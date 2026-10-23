//! Every runtime item has its refinement statement (SPEC.md §17.16, §17.17):
//! `SP-30`.
//!
//! The items are listed from the compiler's table of runtime items
//! (`runtime::Item::all`, every fixed width included), the statements from the
//! theorems `refines_<function>` of `RustRuntime` and `RustStd`; neither list
//! is read from the other. An item is held when exactly one theorem states it,
//! at its profile, on the function the generated code calls for it; `equal`
//! and `compare` are held by the theorems of `SP-25` instead. That the
//! statements are true is `SP-22`'s and `SP-29`'s: Lean accepts them there.

use std::collections::{BTreeMap, BTreeSet};

use lexlean::calculus::rust::runtime::Item;

use crate::support::{self, repo_root};

const CORE: &str = "compiler/src/RustRuntime.lex.tex";
const STD: &str = "compiler/src/RustStd.lex.tex";
const SUPPORT: &str = "compiler/src/RuntimeSupport.lex.tex";

/// The items the generated code calls as trait functions of `RuntimeSupport`,
/// whose meaning `SP-25` states per type: no function of the two modules is
/// their body, so no `refines_` theorem states them.
const TRAIT_ITEMS: [Item; 2] = [Item::Equal, Item::Compare];

/// One refinement statement: the theorem, the function its statement runs,
/// and the profile and runtime item it states the run of.
#[derive(Debug)]
struct Statement {
    theorem: String,
    function: String,
    profile: String,
    item: String,
}

fn word(text: &str) -> &str {
    let end = text
        .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
        .unwrap_or(text.len());
    &text[..end]
}

fn after<'a>(text: &'a str, marker: &str) -> Option<&'a str> {
    text.find(marker).map(|at| &text[at + marker.len()..])
}

/// The refinement statements of a module, read from the statement of each
/// theorem whose name starts with `refines_`.
fn statements(text: &str) -> Result<Vec<Statement>, String> {
    let mut out = Vec::new();
    let mut rest = text;
    while let Some(at) = rest.find("theorem{\n  name: refines_") {
        let block = &rest[at..];
        let end = block.find("\n}\n").map_or(block.len(), |end| end + 3);
        let theorem = word(&block["theorem{\n  name: ".len()..]).to_owned();
        let statement = after(&block[..end], "\n  statement: ")
            .and_then(|line| line.lines().next())
            .unwrap_or_default();
        let function = after(statement, "(RustSemantics::runExport 12 (")
            .and_then(|tail| after(tail, ") \""))
            .and_then(|tail| tail.split('"').next());
        let stated = after(statement, "(RustSemantics::runItem (ctor RustSyntax::Profile.")
            .map(|tail| (word(tail), after(tail, ") (ctor RustSyntax::Item.")));
        let (Some(function), Some((profile, Some(item)))) = (function, stated) else {
            return Err(format!(
                "`{theorem}` does not state the run of a function against a runtime item"
            ));
        };
        let constructor = word(item);
        let item = match after(item, constructor)
            .and_then(|tail| tail.strip_prefix(" (ctor TargetSyntax::IntKind."))
        {
            Some(width) => format!("{constructor} {}", word(width)),
            None => constructor.to_owned(),
        };
        out.push(Statement {
            theorem,
            function: function.to_owned(),
            profile: profile.to_owned(),
            item,
        });
        rest = &block[end..];
    }
    Ok(out)
}

/// The runtime item as the statements write it: the constructor of
/// `RustSyntax.Item` and, for a fixed-width item, its width.
fn key(item: Item) -> String {
    let debug = format!("{item:?}");
    let variant = debug.split('(').next().unwrap_or_default();
    let constructor = format!("{}{}", variant[..1].to_ascii_lowercase(), &variant[1..]);
    match item.width() {
        Some(kind) => format!("{constructor} {}", kind.name()),
        None => constructor,
    }
}

const fn profile(item: Item) -> &'static str {
    if item.heap() {
        "std"
    } else {
        "core"
    }
}

/// The names of the theorems of `SP-25`'s modules.
fn trait_theorems(support_text: &str, keys: &[String]) -> BTreeSet<String> {
    std::iter::once(support_text)
        .chain(keys.iter().map(String::as_str))
        .flat_map(|text| {
            support::module_json(text)["declarations"]
                .as_array()
                .expect("declarations")
                .iter()
                .filter(|declaration| declaration["kind"] == "theorem")
                .filter_map(|declaration| declaration["name"].as_str().map(str::to_owned))
                .collect::<Vec<_>>()
        })
        .collect()
}

/// Whether every runtime item is held: stated by exactly one refinement
/// theorem of the module of its profile, on the function `path` says the
/// generated code calls for it, or a trait item with `SP-25`'s theorems. Every
/// item that is not held is named.
fn held(
    core: &str,
    std: &str,
    path: &dyn Fn(Item) -> String,
    trait_theorems: &BTreeSet<String>,
) -> Result<usize, String> {
    let mut stated: BTreeMap<String, Vec<(Statement, &str)>> = BTreeMap::new();
    for (module, text) in [("core", core), ("std", std)] {
        for statement in statements(text)? {
            stated
                .entry(statement.item.clone())
                .or_default()
                .push((statement, module));
        }
    }
    let items = Item::all();
    let keys: BTreeSet<String> = items.iter().map(|item| key(*item)).collect();
    let mut errors = Vec::new();
    for (item, held_by) in &stated {
        if !keys.contains(item) {
            for (statement, _) in held_by {
                errors.push(format!(
                    "`{}` states `{item}`, which is not a runtime item",
                    statement.theorem
                ));
            }
        }
    }
    let mut count = 0;
    for item in items {
        let name = key(item);
        if TRAIT_ITEMS.contains(&item) {
            if !trait_theorems
                .iter()
                .any(|theorem| theorem.starts_with(&format!("{}_", item.path())))
            {
                errors.push(format!(
                    "`{name}` is a trait item without the theorems of SP-25"
                ));
            }
            continue;
        }
        match stated.get(&name).map(Vec::as_slice) {
            None | Some([]) => errors.push(format!(
                "`{name}` is a runtime item with no refinement statement"
            )),
            Some([(statement, module)]) => {
                let called = path(item);
                if statement.function != called {
                    errors.push(format!(
                        "`{}` states `{name}` on `{}`, but the generated code calls `{called}` for it",
                        statement.theorem, statement.function
                    ));
                }
                if statement.profile != profile(item) || *module != profile(item) {
                    errors.push(format!(
                        "`{}` states `{name}` at the `{}` profile in the `{module}` module; the item is of `{}`",
                        statement.theorem,
                        statement.profile,
                        profile(item)
                    ));
                }
                count += 1;
            }
            Some(many) => errors.push(format!(
                "`{name}` has {} refinement statements: {}",
                many.len(),
                many.iter()
                    .map(|(statement, _)| format!("`{}`", statement.theorem))
                    .collect::<Vec<_>>()
                    .join(", ")
            )),
        }
    }
    if errors.is_empty() {
        Ok(count)
    } else {
        Err(errors.join("; "))
    }
}

fn read(path: &str) -> String {
    std::fs::read_to_string(repo_root().join(path).as_std_path())
        .unwrap_or_else(|error| panic!("{path}: {error}"))
}

/// The module with the theorem `name` left out.
fn without(text: &str, name: &str) -> String {
    let start = text
        .find(&format!("theorem{{\n  name: {name},"))
        .unwrap_or_else(|| panic!("the theorem `{name}`"));
    let end = start + text[start..].find("\n}\n").expect("its end") + 3;
    format!("{}{}", &text[..start], &text[end..])
}

/// The module with `from` replaced by `to` inside the theorem `name` only.
fn within(text: &str, name: &str, from: &str, to: &str) -> String {
    let start = text
        .find(&format!("theorem{{\n  name: {name},"))
        .unwrap_or_else(|| panic!("the theorem `{name}`"));
    let end = start + text[start..].find("\n}\n").expect("its end") + 3;
    let block = &text[start..end];
    let statement = after(block, "\n  statement: ")
        .and_then(|line| line.lines().next())
        .unwrap_or_default();
    assert!(
        statement.contains(from),
        "`{name}`: the plant site is its statement"
    );
    format!("{}{}{}", &text[..start], block.replacen(from, to, 1), &text[end..])
}

/// Run the case of `SP-30`.
pub fn run(id: &str) {
    assert_eq!(id, "SP-30");
    let core = read(CORE);
    let std = read(STD);
    let traits = trait_theorems(&read(SUPPORT), &super::runtime_semantics::key_modules());
    let path = |item: Item| item.path();
    let count = held(&core, &std, &path, &traits).expect("every runtime item is held");
    assert_eq!(
        count + TRAIT_ITEMS.len(),
        Item::all().len(),
        "each runtime item but the trait items has its one statement"
    );
    assert!(count > 100, "the table of runtime items is not empty");

    // A runtime item with no refinement statement.
    let error = held(&core, &without(&std, "refines_parse_u8"), &path, &traits)
        .expect_err("an item with no statement");
    assert!(
        error.contains("`parseFixed u8` is a runtime item with no refinement statement"),
        "{error}"
    );
    let error = held(&without(&core, "refines_nat_add"), &std, &path, &traits)
        .expect_err("a core item with no statement");
    assert!(
        error.contains("`natAdd` is a runtime item with no refinement statement"),
        "{error}"
    );
    // A refinement statement of another item than its function is called for.
    let other = within(
        &std,
        "refines_format_u8",
        "(ctor RustSyntax::Item.formatFixed (ctor TargetSyntax::IntKind.u8))",
        "(ctor RustSyntax::Item.formatFixed (ctor TargetSyntax::IntKind.u16))",
    );
    let error = held(&core, &other, &path, &traits).expect_err("a statement of another item");
    assert!(
        error.contains("`formatFixed u8` is a runtime item with no refinement statement"),
        "{error}"
    );
    assert!(
        error.contains(
            "`formatFixed u16` has 2 refinement statements: `refines_format_u16`, `refines_format_u8`"
        ) || error.contains(
            "`formatFixed u16` has 2 refinement statements: `refines_format_u8`, `refines_format_u16`"
        ),
        "{error}"
    );
    // A statement at the other profile.
    let elsewhere = within(
        &std,
        "refines_join",
        "(ctor RustSyntax::Profile.std)",
        "(ctor RustSyntax::Profile.core)",
    );
    let error = held(&core, &elsewhere, &path, &traits).expect_err("a statement at core");
    assert!(
        error.contains("`refines_join` states `join` at the `core` profile in the `std` module; the item is of `std`"),
        "{error}"
    );
    // The generated code calling another function for an item than the one
    // its statement runs: the integer formatting called as `format_i64`.
    let called = |item: Item| match item {
        Item::FormatInt => "format_i64".to_owned(),
        other => other.path(),
    };
    let error = held(&core, &std, &called, &traits).expect_err("an item called elsewhere");
    assert!(
        error.contains(
            "`refines_format_int` states `formatInt` on `format_int`, but the generated code calls `format_i64` for it"
        ),
        "{error}"
    );
    // A trait item without the theorems of SP-25.
    let bare = trait_theorems(
        &read(SUPPORT).replace("name: equal_", "name: sameness_"),
        &super::runtime_semantics::key_modules()
            .iter()
            .map(|text| text.replace("name: equal_", "name: sameness_"))
            .collect::<Vec<_>>(),
    );
    let error = held(&core, &std, &path, &bare).expect_err("a trait item without theorems");
    assert!(
        error.contains("`equal` is a trait item without the theorems of SP-25"),
        "{error}"
    );
}
