//! The authoring API (SPEC.md §24.8): read, write, convert, and assemble the
//! `.lex.tex` source of a module without a project.
//!
//! Everything the repository's own tools do with a module's source (the
//! formatter, the identity of the embedded tree, the conformance suite) goes
//! through the functions here, so a program that produces a source is held to
//! the same text a person writes: a module is written in the authoring
//! notation (§17.21) or as its canonical JSON data form, and the two read to
//! one value. Nothing here links or elaborates a module; that is the engine's
//! (§24.1), and the diagnostics of a source that does not link are its.

use serde_json::Value;

use crate::artifact::canonical_json::Json;
use crate::surface::module::{locate, Kind, Located, CORE, SEMANTIC, TOOLING};

/// The kinds of module that have an authoring notation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleKind {
    /// A semantic module (SPEC.md §17.11, §17.12).
    Semantic,
    /// A compiler-tooling module (SPEC.md §17.20).
    Tooling,
    /// A native core module (SPEC.md §17.10), a library of kernel terms.
    Core,
}

impl ModuleKind {
    fn kind(self) -> &'static Kind {
        match self {
            Self::Semantic => &SEMANTIC,
            Self::Tooling => &TOOLING,
            Self::Core => &CORE,
        }
    }
}

/// Why a source could not be read, written, or assembled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthoringError {
    /// What is wrong.
    pub message: String,
    /// The byte offset in the source the error is about, when it has one.
    pub offset: Option<usize>,
}

impl std::fmt::Display for AuthoringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.offset {
            Some(offset) => write!(f, "{} (at byte {offset})", self.message),
            None => f.write_str(&self.message),
        }
    }
}

impl std::error::Error for AuthoringError {}

fn error(message: impl Into<String>, offset: Option<usize>) -> AuthoringError {
    AuthoringError {
        message: message.into(),
        offset,
    }
}

fn found(text: &str) -> Result<Located, AuthoringError> {
    locate(text).ok_or_else(|| {
        error(
            "the source states no semantic, tooling, or core module",
            None,
        )
    })
}

/// The canonical JSON of `data`.
fn canonical(data: &Value) -> Result<Json, AuthoringError> {
    Json::parse(
        serde_json::to_string(data)
            .map_err(|failure| error(failure.to_string(), None))?
            .as_bytes(),
    )
    .map_err(|failure| {
        error(
            format!("the module data is not canonical JSON: {failure}"),
            None,
        )
    })
}

/// `json` is a module of the schema the notation of `kind` names: the tag the
/// language implies. A module under another tag (a language-1.1 module) has no
/// notation, because the notation does not write the tag.
fn require_spec(kind: &Kind, json: &Json) -> Result<(), AuthoringError> {
    let spec = (kind.spec)().ok_or_else(|| error("the notation is a language-1.2 form", None))?;
    match json {
        Json::Obj(members) if members.get("spec") == Some(&Json::Str(spec.to_owned())) => Ok(()),
        _ => Err(error(
            format!("the module is not a `{spec}` module, the schema its notation names"),
            None,
        )),
    }
}

/// The kind of the module of `text`, or `None` when it states none.
#[must_use]
pub fn module_kind(text: &str) -> Option<ModuleKind> {
    locate(text).map(|located| {
        if located.kind.is(&SEMANTIC) {
            ModuleKind::Semantic
        } else if located.kind.is(&TOOLING) {
            ModuleKind::Tooling
        } else {
            ModuleKind::Core
        }
    })
}

/// Whether the module of `text` is written in the authoring notation and
/// reads (it is not the `\semanticdata` form, and it is well formed).
#[must_use]
pub fn is_notation(text: &str) -> bool {
    locate(text).is_some_and(|located| {
        located.data.is_none()
            && (located.kind.spec)().is_some_and(|spec| {
                located
                    .kind
                    .data_text(&text[located.body.0..located.body.1], spec)
                    .is_ok()
            })
    })
}

/// The module of `text` as JSON, whichever form it is written in. A module in
/// the notation is read as the compiler reads it, under the schema tag
/// language 1.2 implies.
///
/// # Errors
///
/// A source with no module, a module whose data is not JSON, or notation that
/// is not well formed (the error carries the offset in `text`).
pub fn read_module(text: &str) -> Result<Value, AuthoringError> {
    let located = found(text)?;
    if let Some((start, end)) = located.data {
        return serde_json::from_str(&text[start..end]).map_err(|failure| {
            error(
                format!("the module data is not JSON: {failure}"),
                Some(start),
            )
        });
    }
    let spec =
        (located.kind.spec)().ok_or_else(|| error("the notation is a language-1.2 form", None))?;
    let module = located
        .kind
        .data_text(&text[located.body.0..located.body.1], spec)
        .map_err(|failure| {
            error(
                format!("the notation is not well formed: {}", failure.message),
                Some(located.body.0 + failure.offset),
            )
        })?;
    serde_json::from_str(&module).map_err(|failure| error(failure.to_string(), None))
}

/// `text` with its module's data replaced by `data`, written in the form the
/// module already uses: canonical JSON for the data form, the canonical
/// notation for the notation. A rewrite never changes which form a source is
/// written in.
///
/// # Errors
///
/// A source with no module, or `data` that is not a module of its kind.
pub fn write_module(text: &str, data: &Value) -> Result<String, AuthoringError> {
    let located = found(text)?;
    let json = canonical(data)?;
    if let Some((start, end)) = located.data {
        return Ok(format!(
            "{}{}{}",
            &text[..start],
            json.to_canonical_string(),
            &text[end..]
        ));
    }
    require_spec(located.kind, &json)?;
    let printed = (located.kind.print)(&json)
        .map_err(|failure| error(format!("the module has no notation: {failure}"), None))?;
    Ok(format!(
        "{}\n{printed}{}",
        &text[..located.body.0],
        &text[located.body.1..]
    ))
}

/// `text` with its module written as `\semanticdata{…}` (`\toolingdata{…}`),
/// the canonical JSON the compiler links: the form a module had before the
/// notation existed.
///
/// # Errors
///
/// A source with no well-formed module.
pub fn to_data_form(text: &str) -> Result<String, AuthoringError> {
    let located = found(text)?;
    // A module in the notation is read to its canonical JSON text directly: a
    // library of kernel terms is tens of megabytes, which a value of the whole
    // module would hold several times over.
    let data = if located.data.is_none() {
        let spec = (located.kind.spec)()
            .ok_or_else(|| error("the notation is a language-1.2 form", None))?;
        located
            .kind
            .data_text(&text[located.body.0..located.body.1], spec)
            .map_err(|failure| {
                error(
                    format!("the notation is not well formed: {}", failure.message),
                    Some(located.body.0 + failure.offset),
                )
            })?
    } else {
        canonical(&read_module(text)?)?.to_canonical_string()
    };
    Ok(format!(
        "{}\n{}{{{data}}}\n{}",
        &text[..located.body.0],
        located.kind.control,
        &text[located.body.1..]
    ))
}

/// `text` with its module written in the canonical notation: what `fmt`
/// writes for a language-1.2 module.
///
/// # Errors
///
/// A source with no well-formed module, or a module that is not of the schema
/// the notation names (a language-1.1 module has no notation).
pub fn to_notation(text: &str) -> Result<String, AuthoringError> {
    let located = found(text)?;
    let json = canonical(&read_module(text)?)?;
    require_spec(located.kind, &json)?;
    let printed = (located.kind.print)(&json)
        .map_err(|failure| error(format!("the module has no notation: {failure}"), None))?;
    Ok(format!(
        "{}\n{printed}{}",
        &text[..located.body.0],
        &text[located.body.1..]
    ))
}

/// The byte range in `text` of every declaration of its module written in the
/// notation, in order: what a source map relates to a declaration's
/// rendering. `None` for a module written as data.
///
/// # Errors
///
/// A source with no module, or notation that is not well formed.
pub fn declaration_spans(text: &str) -> Result<Option<Vec<(usize, usize)>>, AuthoringError> {
    let located = found(text)?;
    if located.data.is_some() {
        return Ok(None);
    }
    let body = &text[located.body.0..located.body.1];
    let spans = if located.kind.is(&TOOLING) {
        crate::surface::tooling::declaration_spans(body)
    } else if located.kind.is(&CORE) {
        crate::surface::core::declaration_spans(body)
    } else {
        crate::surface::semantic::declaration_spans(body)
    }
    .map_err(|failure| {
        error(
            format!("the notation is not well formed: {}", failure.message),
            Some(located.body.0 + failure.offset),
        )
    })?;
    Ok(Some(
        spans
            .into_iter()
            .map(|(start, end)| (located.body.0 + start, located.body.0 + end))
            .collect(),
    ))
}

/// The header of a module's source: its name, the glossary packages it uses,
/// the modules it imports, and its title.
pub struct SourceHeader<'a> {
    /// The module's name.
    pub name: &'a str,
    /// The glossary packages, as `package@version`.
    pub glossaries: &'a [&'a str],
    /// The modules it imports, by name.
    pub imports: &'a [&'a str],
    /// The title.
    pub title: &'a str,
}

/// The whole `.lex.tex` source of a module of `kind` with `data`, written in
/// the canonical notation: the header, then the module, then the closing.
///
/// # Errors
///
/// `data` that is not a module of its kind.
pub fn assemble(
    header: &SourceHeader<'_>,
    kind: ModuleKind,
    data: &Value,
) -> Result<String, AuthoringError> {
    let kind = kind.kind();
    let json = canonical(data)?;
    require_spec(kind, &json)?;
    let printed = (kind.print)(&json)
        .map_err(|failure| error(format!("the module has no notation: {failure}"), None))?;
    let mut out = format!("\\begin{{lexlean}}{{{}}}\n", header.name);
    for glossary in header.glossaries {
        out.push_str(&format!("\\useglossary{{{glossary}}}\n"));
    }
    for import in header.imports {
        out.push_str(&format!("\\importmodule{{{import}}}\n"));
    }
    out.push_str(&format!(
        "\\title{{{}}}\n\\begin{{{}}}\n{printed}\\end{{{}}}\n\\end{{lexlean}}\n",
        header.title, kind.environment, kind.environment
    ));
    Ok(out)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{
        assemble, declaration_spans, is_notation, module_kind, read_module, to_data_form,
        to_notation, write_module, ModuleKind, SourceHeader,
    };

    fn module() -> serde_json::Value {
        json!({"spec": "lexlean/semantic-module/2", "declarations": [
            {"kind": "theorem", "name": "t", "parameters": [],
             "statement": {"kind": "bool", "value": true}, "proof": {"kind": "decide"}}
        ]})
    }

    fn header() -> SourceHeader<'static> {
        SourceHeader {
            name: "Main",
            glossaries: &["lexlean.std.nat@1.2.0"],
            imports: &["Other"],
            title: "T",
        }
    }

    /// A source is assembled in the notation and read back to its data;
    /// the data form is the same module; and a rewrite keeps the form.
    #[test]
    fn a_source_is_assembled_read_and_rewritten_in_its_own_form() {
        let source = assemble(&header(), ModuleKind::Semantic, &module()).expect("assembles");
        assert!(source.contains("\\importmodule{Other}\n"), "{source}");
        assert!(source.contains("theorem{"), "{source}");
        assert_eq!(module_kind(&source), Some(ModuleKind::Semantic));
        assert!(is_notation(&source));
        assert_eq!(read_module(&source).expect("reads"), module());

        let data = to_data_form(&source).expect("the data form");
        assert!(!is_notation(&data));
        assert!(data.contains("\\semanticdata{{\"declarations\":"), "{data}");
        assert_eq!(read_module(&data).expect("reads"), module());
        assert_eq!(to_notation(&data).expect("the notation"), source);

        let mut changed = module();
        changed["declarations"][0]["name"] = json!("u");
        let rewritten = write_module(&source, &changed).expect("rewrites");
        assert!(is_notation(&rewritten), "{rewritten}");
        assert_eq!(
            read_module(&rewritten).expect("reads")["declarations"][0]["name"],
            "u"
        );
        let rewritten = write_module(&data, &changed).expect("rewrites");
        assert!(!is_notation(&rewritten), "{rewritten}");

        let spans = declaration_spans(&source)
            .expect("spans")
            .expect("a notation");
        assert_eq!(spans.len(), 1);
        assert!(source[spans[0].0..spans[0].1].starts_with("theorem{"));
        assert_eq!(declaration_spans(&data).expect("spans"), None);
    }

    /// A module under a tag the notation does not name has no notation.
    #[test]
    fn a_module_under_another_tag_has_no_notation() {
        let mut other = module();
        other["spec"] = json!("lexlean/semantic-module/1");
        assert!(assemble(&header(), ModuleKind::Semantic, &other).is_err());
        let source = assemble(&header(), ModuleKind::Semantic, &module()).expect("assembles");
        assert!(write_module(&source, &other).is_err());
        let data = to_data_form(&source).expect("the data form");
        let written = write_module(&data, &other).expect("the data form carries any tag");
        assert!(to_notation(&written).is_err());
    }

    /// What is not a module is an error with a position, never a panic.
    #[test]
    fn what_is_not_a_module_is_an_error() {
        assert!(read_module("\\begin{lexlean}{M}\n\\end{lexlean}\n").is_err());
        let source = assemble(&header(), ModuleKind::Semantic, &module()).expect("assembles");
        let broken = source.replace("name: t", "name: ");
        let failure = read_module(&broken).expect_err("malformed");
        assert!(
            failure.offset.is_some_and(|offset| offset <= broken.len()),
            "{failure}"
        );
        assert!(assemble(&header(), ModuleKind::Semantic, &json!(3)).is_err());
        assert!(!is_notation(&broken));
    }
}
