//! The reference renderings of target programs to Rust (SPEC.md §17.14,
//! §17.16).
//!
//! A program renders to one safe Rust 2021 library crate in one of the two
//! machine profiles of §17.13. `rust-core` is `#![no_std]` and allocates
//! nothing: it admits only types whose values have a fixed size, so a
//! string, a byte string, a list, or a type that contains itself is refused.
//! `rust-std` shares every heap value behind `Rc`: a string, a byte string,
//! and each list cell are immutable shared buffers, and an ADT field or a
//! closure capture is boxed exactly where its type contains its owner. In
//! both, using a local clones a handle, never a structure, and building or
//! taking apart a list cell is constant work, so every evaluation step is
//! realized by work bounded by its charge.
//!
//! Rendering lowers the program to the closed Rust AST ([`ast`]), checks it
//! ([`validate`]), and prints it into canonical bytes. A function that
//! cannot overflow returns its value; one that can returns `R<T>` and every
//! call to it propagates with `?`. Packages ([`package`]) add exported
//! functions, a Cargo manifest, and provenance. A rendered function never
//! panics on a valid program; fuel and stack depth are outside the
//! observable contract. Rendering refuses a program it cannot render
//! faithfully.

pub mod ast;
mod lower;
pub mod package;
pub mod runtime;
pub mod validate;

use std::collections::BTreeSet;

use super::{Program, Ty, Value};
use lower::{Lowering, Owner};

/// A machine profile of §17.13.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Profile {
    /// `rust-core`: the core library only, no heap allocation.
    Core,
    /// `rust-std`: the standard library, heap allocation.
    Std,
}

impl Profile {
    /// Both profiles.
    pub const ALL: [Self; 2] = [Self::Core, Self::Std];

    /// The target identifier of the production registry.
    #[must_use]
    pub const fn target(self) -> &'static str {
        match self {
            Self::Core => "rust-core",
            Self::Std => "rust-std",
        }
    }

    /// The profile of a target identifier.
    #[must_use]
    pub fn named(target: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|profile| profile.target() == target)
    }
}

/// The calculus elements a lowering realizes: the program's own, and the
/// structural realizations it uses (§17.14): `function` always, `overflow`
/// when a primitive or successor can fail, and `indirection` when a type
/// holds itself.
fn elements(program: &Program, lowering: &Lowering<'_>) -> BTreeSet<String> {
    let mut out = super::realization::program_elements(program);
    out.insert("function".to_owned());
    // Declaring an ADT uses the ADT type former, even where no stated type
    // names the declaration.
    if !program.adts.is_empty() {
        out.insert("type:adt".to_owned());
    }
    const FAILING: [&str; 9] = [
        "prim:nat_add",
        "prim:nat_mul",
        "prim:int_add",
        "prim:int_sub",
        "prim:int_mul",
        "prim:int_neg",
        "prim:int_quot",
        "prim:parse_decimal",
        "shape:succ",
    ];
    if FAILING.iter().any(|element| out.contains(*element)) {
        out.insert("overflow".to_owned());
    }
    if lowering.indirect() {
        out.insert("indirection".to_owned());
    }
    out
}

/// The calculus elements and structural realizations a valid program's
/// rendering in `profile` may realize, as [`validate::correspond`] checks
/// them.
///
/// # Errors
///
/// Returns the reason the program is invalid.
pub fn realized(program: &Program, profile: Profile) -> Result<BTreeSet<String>, String> {
    Ok(elements(program, &Lowering::new(program, profile)?))
}

/// Lower a valid program to the closed Rust AST of `profile`, checked.
///
/// # Errors
///
/// Returns the reason the program is invalid, has no rendering in
/// `profile`, or lowers to a crate the checks of [`validate`] refuse.
pub fn lower(program: &Program, profile: Profile) -> Result<ast::Crate, String> {
    let mut lowering = Lowering::new(program, profile)?;
    let krate = lowering.lower()?;
    validate::validate(&krate)?;
    validate::correspond(&krate, &elements(program, &lowering))?;
    Ok(krate)
}

/// Which functions of a valid program can overflow, as the renderings
/// type them: a function that can returns `R<T>`, one that cannot returns
/// its value.
///
/// # Errors
///
/// Returns the reason the program is invalid.
pub fn fallible_functions(program: &Program) -> Result<Vec<bool>, String> {
    Ok(Lowering::new(program, Profile::Std)?.fallible)
}

/// Render a valid program to the canonical bytes of a library crate of
/// `profile`.
///
/// # Errors
///
/// As [`lower`].
pub fn render(program: &Program, profile: Profile) -> Result<String, String> {
    Ok(ast::print(&lower(program, profile)?))
}

/// Rust literals of the values of one program in one profile, built through
/// the closed AST and printed by its printer: the one hook a harness that
/// calls a rendered package needs from the renderer (§17.16).
pub struct Literals<'a> {
    lowering: Lowering<'a>,
}

impl<'a> Literals<'a> {
    /// The literals of `program` in `profile`.
    ///
    /// # Errors
    ///
    /// Returns the reason the program is invalid.
    pub fn new(program: &'a Program, profile: Profile) -> Result<Self, String> {
        Ok(Self {
            lowering: Lowering::new(program, profile)?,
        })
    }

    /// The canonical Rust text of `value` at type `ty`.
    ///
    /// # Errors
    ///
    /// Returns the reason the value has no literal at `ty` in the profile.
    pub fn literal(&mut self, value: &Value, ty: &Ty) -> Result<String, String> {
        Ok(ast::print_expr(&self.lowering.value(value, ty)?))
    }

    /// Whether a field of type `ty` of ADT `adt` is boxed in the rendering.
    ///
    /// # Errors
    ///
    /// Returns the reason the profile cannot box it.
    pub fn boxed(&self, adt: u64, ty: &Ty) -> Result<bool, String> {
        self.lowering.boxed(Owner::Adt(adt), ty)
    }
}

/// The observable outcome a rendered harness prints for an outcome of the
/// denotation: the value, or `overflow`; steps are not observable in Rust.
#[must_use]
pub fn observable(outcome: &super::Outcome) -> Option<serde_json::Value> {
    match outcome {
        super::Outcome::Value { value, .. } => serde_json::to_value(value).ok(),
        super::Outcome::Overflow { .. } => Some(serde_json::json!({"kind": "overflow"})),
        super::Outcome::Stuck | super::Outcome::Exhausted => None,
    }
}

/// The source files of the renderer, by name in `calculus/rust/`, in the
/// order [`renderer_digest`] frames them.
pub const RENDERER_FILES: [&str; 6] = [
    "ast.rs",
    "lower.rs",
    "mod.rs",
    "package.rs",
    "runtime.rs",
    "validate.rs",
];

/// The digest LexLean's compiler semantics records of the renderer
/// (`rust_renderer` in `language/semantics-1.2.toml`, §17.16): one frame
/// (§21.1) per source file of [`RENDERER_FILES`], labeled by its name, so a
/// change to any rendering is a change to LexLean's identity.
#[must_use]
pub fn renderer_digest(files: &[(&str, &[u8])]) -> crate::artifact::content_id::Sha256Digest {
    let mut hasher = crate::artifact::content_id::FramedHasher::new("lexlean-rust-renderer-v1");
    for (name, bytes) in files {
        hasher.frame(name, bytes);
    }
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::{renderer_digest, Profile, RENDERER_FILES};

    /// The renderer cannot change without its record in the language-1.2
    /// semantics, so a change to any rendering changes LexLean's
    /// compiler-semantics ID.
    #[test]
    fn the_compiler_semantics_records_the_renderer() {
        let sources: [&[u8]; 6] = [
            include_bytes!("ast.rs"),
            include_bytes!("lower.rs"),
            include_bytes!("mod.rs"),
            include_bytes!("package.rs"),
            include_bytes!("runtime.rs"),
            include_bytes!("validate.rs"),
        ];
        let files: Vec<(&str, &[u8])> = RENDERER_FILES.into_iter().zip(sources).collect();
        let (_, semantics) = crate::embedded::FILES
            .iter()
            .find(|(path, _)| *path == "language/semantics-1.2.toml")
            .expect("the language-1.2 semantics");
        let table: toml::Value = std::str::from_utf8(semantics)
            .expect("UTF-8")
            .parse()
            .expect("TOML");
        let digest = renderer_digest(&files).to_hex();
        assert_eq!(
            table.get("rust_renderer").and_then(toml::Value::as_str),
            Some(digest.as_str()),
            "the renderer changed without its record `rust_renderer` in language/semantics-1.2.toml"
        );
    }

    #[test]
    fn targets_name_their_profiles() {
        for profile in Profile::ALL {
            assert_eq!(Profile::named(profile.target()), Some(profile));
        }
        assert_eq!(Profile::named("rust-wasm"), None);
    }
}
