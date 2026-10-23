//! The kinds of module that have an authoring notation, and where a module
//! is in a source (SPEC.md §17.21).
//!
//! One table serves everything that reads a source's module without linking
//! it: the formatter, the identity of the embedded tree, and the authoring
//! API. A module is the body of one environment, either `\semanticdata{…}`
//! (`\toolingdata{…}`, `\coredata{…}`) holding canonical JSON, or the
//! notation of the module.

use crate::artifact::canonical_json::Json;

use super::SurfaceError;

/// Reads the notation of a module under a schema tag.
pub type Parse = fn(&str, &str) -> Result<Json, SurfaceError>;

/// Writes the notation of a module.
pub type Print = fn(&Json) -> Result<String, SurfaceError>;

/// A kind of module with a notation.
pub struct Kind {
    /// The environment the module is written in.
    pub environment: &'static str,
    /// The control of its data form.
    pub control: &'static str,
    /// The schema tag the language implies for a module in the notation.
    pub spec: fn() -> Option<&'static str>,
    /// Reads the notation.
    pub parse: Parse,
    /// Writes the notation.
    pub print: Print,
}

/// Semantic modules (§17.11, §17.12).
pub const SEMANTIC: Kind = Kind {
    environment: "semanticmodule",
    control: "\\semanticdata",
    spec: || crate::ir::semantic::semantic_module_spec(crate::LANGUAGE_1_2),
    parse: super::semantic::parse_module,
    print: super::semantic::print_module,
};

/// Compiler-tooling modules (§17.20).
pub const TOOLING: Kind = Kind {
    environment: "toolingmodule",
    control: "\\toolingdata",
    spec: || Some(crate::ir::tooling::TOOLING_SPEC),
    parse: super::tooling::parse_module,
    print: super::tooling::print_module,
};

/// Native core modules (§17.10).
pub const CORE: Kind = Kind {
    environment: "coremodule",
    control: "\\coredata",
    spec: || Some(crate::ir::core::CORE_SPEC_2),
    parse: core_parse,
    print: core_print,
};

/// The kinds of module with a notation.
pub const KINDS: [&Kind; 3] = [&SEMANTIC, &TOOLING, &CORE];

fn fail(message: impl Into<String>) -> SurfaceError {
    SurfaceError {
        offset: 0,
        message: message.into(),
    }
}

/// The notation of a core module as the canonical JSON the other kinds read
/// to: the whole module as a value, which the larger modules make costly;
/// [`Kind::data_text`] is the form that avoids it.
fn core_parse(text: &str, spec: &str) -> Result<Json, SurfaceError> {
    let data = super::core::data_text(text, spec)?;
    Json::parse(data.as_bytes()).map_err(fail)
}

fn core_print(module: &Json) -> Result<String, SurfaceError> {
    super::core::print_data(&module.to_canonical_string())
}

/// Where the module of a source is.
pub struct Located {
    /// The kind of the module.
    pub kind: &'static Kind,
    /// The byte range between the environment's markers: the body.
    pub body: (usize, usize),
    /// The byte range of the JSON between the braces of the control, when the
    /// module is written as data; `None` when it is written in the notation.
    pub data: Option<(usize, usize)>,
}

impl Kind {
    /// Whether `other` is the same kind: kinds are identified by the
    /// environment they are written in.
    #[must_use]
    pub fn is(&self, other: &Kind) -> bool {
        self.environment == other.environment
    }

    /// The canonical JSON text of the module a notation reads to: what a
    /// source of this kind written as data holds between the braces of its
    /// control.
    ///
    /// # Errors
    ///
    /// Text that is not the notation of a module.
    pub fn data_text(&self, notation: &str, spec: &str) -> Result<String, SurfaceError> {
        if self.is(&CORE) {
            super::core::data_text(notation, spec)
        } else {
            (self.parse)(notation, spec).map(|module| module.to_canonical_string())
        }
    }

    /// The notation of the module whose canonical JSON text is `data`, when
    /// it is a module under the schema the notation names: `None` for text
    /// that is not (a module of another schema has no notation, §17.21.5).
    #[must_use]
    pub fn notation_of_data(&self, data: &str) -> Option<Result<String, SurfaceError>> {
        let spec = (self.spec)()?;
        if self.is(&CORE) {
            let module: crate::ir::core::CoreModule = serde_json::from_str(data).ok()?;
            return (module.spec == spec).then(|| super::core::print_module(&module));
        }
        let json = Json::parse(data.as_bytes()).ok()?;
        match &json {
            Json::Obj(members) if members.get("spec") == Some(&Json::Str(spec.to_owned())) => {
                Some((self.print)(&json))
            }
            _ => None,
        }
    }

    /// The canonical notation of the module a notation reads to.
    ///
    /// # Errors
    ///
    /// Text that is not the notation of a module.
    pub fn reprint(&self, notation: &str, spec: &str) -> Result<String, SurfaceError> {
        if self.is(&CORE) {
            super::core::print_module(&super::core::parse_module(notation)?)
        } else {
            (self.print)(&(self.parse)(notation, spec)?)
        }
    }

    /// The environment's opening marker.
    #[must_use]
    pub fn begin(&self) -> String {
        format!("\\begin{{{}}}", self.environment)
    }

    /// The environment's closing marker.
    #[must_use]
    pub fn end(&self) -> String {
        format!("\\end{{{}}}", self.environment)
    }
}

/// The control atoms of `text` from byte `at`, each with the byte after it,
/// as the scanner reads them (SPEC.md §12.2): a backslash and ASCII letters, or
/// a backslash and one ASCII nonletter. The escape `\\` is one atom, so the
/// word after it is not a control: a marker a string quotes is not a marker. A
/// backslash before a non-ASCII scalar or the end of the text is no atom (the
/// scanner refuses it), and the search goes on after it.
fn controls(text: &str, mut at: usize) -> impl Iterator<Item = (&str, usize)> + '_ {
    let bytes = text.as_bytes();
    std::iter::from_fn(move || loop {
        let start = at + text.get(at..)?.find('\\')?;
        let mut end = start + 1;
        match bytes.get(end) {
            Some(next) if next.is_ascii_alphabetic() => {
                while bytes.get(end).is_some_and(u8::is_ascii_alphabetic) {
                    end += 1;
                }
            }
            Some(next) if next.is_ascii() => end += 1,
            _ => {
                at = end;
                continue;
            }
        }
        at = end;
        return Some((&text[start..end], end));
    })
}

/// The environment name a `\begin` or `\end` control ending at `at` names,
/// `{name}`, with the byte after its closing brace.
fn environment_name(text: &str, at: usize) -> Option<(&str, usize)> {
    let rest = text.get(at..)?.strip_prefix('{')?;
    let close = rest.find('}')?;
    Some((&rest[..close], at + 1 + close + 1))
}

/// Find the module of `text`: the environment the structural parser opens
/// (SPEC.md §15.1, §17.21.1), which is the first environment after
/// `\begin{lexlean}`, and its body, which runs to the first `\end` control
/// after it. Both are read over the control atoms of the text, not by a search
/// of it, so a marker that a string quotes (`\\begin{…}`, an escaped backslash
/// and a word) is never taken for a module's, whatever kind it names. `None`
/// when `text` opens no environment of a kind, or when the first `\end` after
/// the module's opening does not close it (the parser refuses that source
/// with `LLP2003`).
#[must_use]
pub fn locate(text: &str) -> Option<Located> {
    let mut atoms = controls(text, 0);
    let (kind, start) = loop {
        let (control, after) = atoms.next()?;
        if control != "\\begin" {
            continue;
        }
        let (name, after) = environment_name(text, after)?;
        if name == "lexlean" {
            continue;
        }
        break (
            KINDS.into_iter().find(|kind| kind.environment == name)?,
            after,
        );
    };
    let end = loop {
        let (control, after) = atoms.next()?;
        if control == "\\end" {
            let (name, _) = environment_name(text, after)?;
            if name != kind.environment {
                return None;
            }
            break after - control.len();
        }
    };
    let body = text.get(start..end)?;
    let data = body
        .trim_start()
        .strip_prefix(kind.control)
        .and_then(|rest| {
            let open = rest.trim_start().strip_prefix('{')?;
            let close = open.trim_end().strip_suffix('}')?;
            // The offsets of `close` within `text`.
            let from = close.as_ptr() as usize - text.as_ptr() as usize;
            Some((from, from + close.len()))
        });
    Some(Located {
        kind,
        body: (start, end),
        data,
    })
}

#[cfg(test)]
mod tests {
    use super::locate;

    /// Both forms are found, with the range of what each holds.
    #[test]
    fn the_module_of_a_source_is_found_in_either_form() {
        let data = "\\begin{lexlean}{M}\n\\begin{semanticmodule}\n\\semanticdata{{\"a\":1}}\n\\end{semanticmodule}\n\\end{lexlean}\n";
        let found = locate(data).expect("a module");
        assert_eq!(found.kind.environment, "semanticmodule");
        let (start, end) = found.data.expect("the data form");
        assert_eq!(&data[start..end], "{\"a\":1}");
        let notation =
            "\\begin{lexlean}{M}\n\\begin{toolingmodule}\n{declarations: []}\n\\end{toolingmodule}\n\\end{lexlean}\n";
        let found = locate(notation).expect("a module");
        assert_eq!(found.kind.environment, "toolingmodule");
        assert!(found.data.is_none());
        assert_eq!(
            &notation[found.body.0..found.body.1],
            "\n{declarations: []}\n"
        );
        assert!(locate("\\begin{lexlean}{M}\n\\end{lexlean}\n").is_none());
        assert!(locate("\\begin{semanticmodule}").is_none());
    }

    /// A module is the environment the parser opens: a marker a string quotes
    /// (an escaped backslash and the marker's word) is not one, whichever kind
    /// it names and whether or not its end is quoted too (#241).
    #[test]
    fn a_quoted_marker_is_not_a_module() {
        let tooling = "\\begin{lexlean}{M}\n\\title{t}\n\\begin{toolingmodule}\n{declarations: [\"\\\\begin{semanticmodule}\"]}\n\\end{toolingmodule}\n\\end{lexlean}\n";
        let found = locate(tooling).expect("a module");
        assert_eq!(found.kind.environment, "toolingmodule");
        assert_eq!(
            &tooling[found.body.0..found.body.1],
            "\n{declarations: [\"\\\\begin{semanticmodule}\"]}\n"
        );
        let both = tooling.replace(
            "\\\\begin{semanticmodule}",
            "\\\\begin{semanticmodule}\\\\end{semanticmodule}",
        );
        assert_eq!(
            locate(&both).expect("a module").kind.environment,
            "toolingmodule"
        );
        let core = "\\begin{lexlean}{M}\n\\title{t}\n\\begin{coremodule}\nx := \"\\\\begin{toolingmodule}\\\\end{toolingmodule}\"\n\\end{coremodule}\n\\end{lexlean}\n";
        assert_eq!(
            locate(core).expect("a module").kind.environment,
            "coremodule"
        );
        let semantic = "\\begin{lexlean}{M}\n\\title{t}\n\\begin{semanticmodule}\nx := \"\\\\begin{coremodule}\\\\end{semanticmodule}\"\n\\end{semanticmodule}\n\\end{lexlean}\n";
        let found = locate(semantic).expect("a module");
        assert_eq!(found.kind.environment, "semanticmodule");
        assert!(semantic[found.body.1..].starts_with("\\end{semanticmodule}\n\\end{lexlean}"));
        // An end marker the body does not escape closes the environment
        // there, as it does for the parser, which refuses the source.
        let unescaped = tooling.replace("\\\\begin{semanticmodule}", "\\end{semanticmodule}");
        assert!(locate(&unescaped).is_none());
        // A module of no kind first: the parser reads its blocks, not a
        // module a later marker names.
        assert!(locate("\\begin{lexlean}{M}\n\\title{t}\n\\begin{definition}\n\\end{definition}\n\\begin{semanticmodule}\n\\end{semanticmodule}\n\\end{lexlean}\n").is_none());
    }
}
