//! Certificate E as kernel-term data (SPEC.md §17.17, `SP-14`).
//!
//! Certificate E of one rendering of a root states that the crate's root,
//! invoked on the encoded arguments, realizes certificate A's observation of
//! the root: it composes certificate A (the root's semantics) with
//! certificate B (the crate's simulation of the lowered program) through the
//! library's `compose` theorem, after proving every encoded argument well
//! typed for the crate. Here the module is built as a native core module: a
//! theorem row per document type (`__wt_k`, the well-typedness of its
//! encoder) and the `root` row, each a kernel term whose every argument is
//! written, the lemma types read from the native preservation library and
//! instantiated one binder at a time ([`super::super::kernel`]).
//!
//! What the text generator spells for Lean's elaborator, this generator
//! writes out: the implicit program, crate, and flags of every lemma, the
//! element types and encoders of the container lemmas, the field values and
//! types of a document, the environment the entry binds, and the observation
//! `compose` relates. An `rfl` is `Eq.refl` at the equation the lemma states.
//! The statements are the text generator's, so the kernel term each theorem
//! states is the one the text elaborated to.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use super::super::kernel::{Arg, Builder, Term};
use super::super::preserve::{self, CERTIFICATE_AXIOMS};
use super::{components, Gen, Nested, SYNTAX};
use crate::calculus::Ty;
use crate::ir::core::{
    CoreAxiomPolicy, CoreBinderInfo, CoreDeclKind, CoreDeclaration, CoreModule, CoreTransparency,
    CORE_SPEC_2,
};
use crate::ir::semantic::{MemberRef, SemanticParameter, SemanticType};

const LIB: &str = "LexLeanPreservation";
const RUST: &str = "LexLeanPreservation.Rust";
/// The library module certificate E imports, which imports the rest.
pub(super) const COMPOSE_MODULE: &str = "LexLeanPreservation.Compose";

/// The builder of one certificate E module.
struct Composer<'g, 'a> {
    gen: &'g Gen<'a>,
    b: Builder,
    module_a: String,
    module_b: String,
    name: String,
    /// `module_b.program`, `module_b.krate`, `module_b.flags`.
    p: Term,
    c: Term,
    a: Term,
    /// The lemma heads and imported types by name.
    lemmas: HashMap<String, (Term, Term)>,
    /// The axioms of each row built so far, by row name, for the rows that
    /// apply them.
    row_axioms: BTreeMap<String, BTreeSet<String>>,
    /// The names of the module's own rows a term applies, since the last
    /// [`Self::take_references`].
    references: BTreeSet<String>,
}

/// One row of the module.
struct Row {
    name: String,
    ty: Term,
    value: Term,
    axioms: BTreeSet<String>,
}

impl<'g, 'a> Composer<'g, 'a> {
    fn new(gen: &'g Gen<'a>, module_a: &str, module_b: &str, name: &str) -> Self {
        let mut b = Builder::new();
        let p = b.konst(&format!("{module_b}.program"));
        let c = b.konst(&format!("{module_b}.krate"));
        let a = b.konst(&format!("{module_b}.flags"));
        Self {
            gen,
            b,
            module_a: module_a.to_owned(),
            module_b: module_b.to_owned(),
            name: name.to_owned(),
            p,
            c,
            a,
            lemmas: HashMap::new(),
            row_axioms: BTreeMap::new(),
            references: BTreeSet::new(),
        }
    }

    // --- Names ------------------------------------------------------------

    fn syntax(&mut self, name: &str) -> Term {
        self.b.konst(&format!("{SYNTAX}.{name}"))
    }

    fn library(&mut self, name: &str) -> Term {
        self.b.konst(&format!("{LIB}.{name}"))
    }

    fn rust(&mut self, name: &str) -> Term {
        self.b.konst(&format!("{RUST}.{name}"))
    }

    fn own(&mut self, name: &str) -> Term {
        let full = format!("{}.{name}", self.name);
        self.references.insert(full.clone());
        self.b.konst(&full)
    }

    fn of_a(&mut self, name: &str) -> Term {
        self.b.konst(&format!("{}.{name}", self.module_a))
    }

    fn take_references(&mut self) -> BTreeSet<String> {
        std::mem::take(&mut self.references)
    }

    /// The head and type of a library lemma, its type imported from the
    /// native library data.
    fn lemma(&mut self, name: &str) -> Result<(Term, Term), String> {
        if let Some(&found) = self.lemmas.get(name) {
            self.b.note_lemma(name);
            return Ok(found);
        }
        let (module, declaration) = preserve::native_declaration(name)?;
        if !declaration.levels.is_empty() {
            return Err(format!(
                "`{name}` has universe parameters, which a certificate does not instantiate"
            ));
        }
        let ty = self.b.import(module, declaration.r#type);
        let head = self.b.konst(name);
        self.lemmas.insert(name.to_owned(), (head, ty));
        self.b.note_lemma(name);
        Ok((head, ty))
    }

    /// A library lemma applied to `args`.
    fn apply(&mut self, name: &str, args: &[Arg]) -> Result<Term, String> {
        let (head, ty) = self.lemma(name)?;
        self.b
            .apply_pi(head, ty, args)
            .map(|applied| applied.term)
            .map_err(|reason| format!("`{name}`: {reason}"))
    }

    fn rust_lemma(&mut self, name: &str, args: &[Arg]) -> Result<Term, String> {
        self.apply(&format!("{RUST}.{name}"), args)
    }

    // --- Types ------------------------------------------------------------

    /// The Lean type of a closed source type, as a kernel term.
    fn lean_type(&mut self, ty: &SemanticType) -> Result<Term, String> {
        Ok(match ty {
            SemanticType::Nat => self.b.konst("Nat"),
            SemanticType::Bool => self.b.konst("Bool"),
            SemanticType::Unit => self.b.konst("Unit"),
            SemanticType::Int => self.b.konst("Int"),
            SemanticType::Int8 => self.b.konst("Int8"),
            SemanticType::Int16 => self.b.konst("Int16"),
            SemanticType::Int32 => self.b.konst("Int32"),
            SemanticType::Int64 => self.b.konst("Int64"),
            SemanticType::UInt8 => self.b.konst("UInt8"),
            SemanticType::UInt16 => self.b.konst("UInt16"),
            SemanticType::UInt32 => self.b.konst("UInt32"),
            SemanticType::UInt64 => self.b.konst("UInt64"),
            SemanticType::String => self.b.konst("String"),
            SemanticType::Bytes => self.b.konst("ByteArray"),
            SemanticType::Ordering => self.b.konst("Ordering"),
            SemanticType::ContractViolation | SemanticType::ReasoningFailure => {
                let bool = self.b.konst("Bool");
                self.b.pair_ty(bool, bool)
            }
            SemanticType::Option { value } => {
                let inner = self.lean_type(value)?;
                let option = self.b.konst0("Option");
                self.b.app(option, inner)
            }
            SemanticType::Result { ok, error } => {
                let error = self.lean_type(error)?;
                let ok = self.lean_type(ok)?;
                let except = self.b.konst00("Except");
                self.b.apps(except, &[error, ok])
            }
            SemanticType::List { element } | SemanticType::Set { element } => {
                let element = self.lean_type(element)?;
                self.b.list_ty(element)
            }
            SemanticType::Product { left, right } => {
                let left = self.lean_type(left)?;
                let right = self.lean_type(right)?;
                self.b.pair_ty(left, right)
            }
            SemanticType::Map { key, value } => {
                let key = self.lean_type(key)?;
                let value = self.lean_type(value)?;
                let entry = self.b.pair_ty(key, value);
                self.b.list_ty(entry)
            }
            SemanticType::Named { member, arguments } => {
                let head = self.document_name(member)?;
                let head = self.b.konst(&head);
                let mut args = Vec::new();
                for argument in arguments {
                    args.push(self.lean_type(argument)?);
                }
                self.b.apps(head, &args)
            }
            SemanticType::Function {
                parameters: _,
                result: _,
            }
            | SemanticType::Type
            | SemanticType::Prop
            | SemanticType::Parameter { name: _ } => {
                return Err(format!(
                    "`{}` has no runtime encoding",
                    self.gen.source.type_text(ty)
                ));
            }
        })
    }

    /// The Lean name of a document type.
    fn document_name(&self, member: &MemberRef) -> Result<String, String> {
        let module = member.module.clone().ok_or("an unanchored document type")?;
        Ok(format!(
            "{}.{}",
            self.gen.source.lean_module(&module)?,
            member.name
        ))
    }

    /// A target type as a kernel term.
    fn target_ty(&mut self, ty: &Ty) -> Term {
        match ty {
            Ty::Unit => self.syntax("Ty.unit"),
            Ty::Bool => self.syntax("Ty.bool"),
            Ty::Nat => self.syntax("Ty.nat"),
            Ty::Int => self.syntax("Ty.int"),
            Ty::Fixed { width } => {
                let fixed = self.syntax("Ty.fixed");
                let kind = self.syntax(&format!("IntKind.{}", width.name()));
                self.b.app(fixed, kind)
            }
            Ty::String => self.syntax("Ty.string"),
            Ty::Bytes => self.syntax("Ty.bytes"),
            Ty::Ordering => self.syntax("Ty.ordering"),
            Ty::Option { value } => {
                let inner = self.target_ty(value);
                let option = self.syntax("Ty.option");
                self.b.app(option, inner)
            }
            Ty::Result { ok, error } => {
                let ok = self.target_ty(ok);
                let error = self.target_ty(error);
                let result = self.syntax("Ty.result");
                self.b.apps(result, &[ok, error])
            }
            Ty::List { element } => {
                let element = self.target_ty(element);
                let list = self.syntax("Ty.list");
                self.b.app(list, element)
            }
            Ty::Pair { left, right } => {
                let left = self.target_ty(left);
                let right = self.target_ty(right);
                let pair = self.syntax("Ty.pair");
                self.b.apps(pair, &[left, right])
            }
            Ty::Adt { index } => {
                let adt = self.syntax("Ty.adt");
                let index = self.b.nat(*index);
                self.b.app(adt, index)
            }
            Ty::Fn { parameters, result } => {
                let ty = self.syntax("Ty");
                let mut items = Vec::new();
                for parameter in parameters {
                    items.push(self.target_ty(parameter));
                }
                let parameters = self.b.list(ty, &items);
                let result = self.target_ty(result);
                let function = self.syntax("Ty.fn");
                self.b.apps(function, &[parameters, result])
            }
        }
    }

    /// The target type a source type is encoded at.
    fn encoded_ty(&mut self, ty: &SemanticType) -> Result<Term, String> {
        let target = self.gen.target_ty(ty)?;
        Ok(self.target_ty(&target))
    }

    // --- Encoders ---------------------------------------------------------

    /// The encoder of a closed source type into target values, as the term
    /// certificate A applies (`enc` and the nested auxiliaries).
    fn encoder(&mut self, ty: &SemanticType) -> Result<Term, String> {
        if let Some(nested) = self.gen.nested.get(&self.gen.source.type_text(ty)) {
            return self.nested_encoder(nested);
        }
        Ok(match ty {
            SemanticType::Nat => self.syntax("Value.nat"),
            SemanticType::Bool => self.syntax("Value.bool"),
            SemanticType::Int => self.syntax("Value.int"),
            SemanticType::Int8 => self.syntax("Value.i8"),
            SemanticType::Int16 => self.syntax("Value.i16"),
            SemanticType::Int32 => self.syntax("Value.i32"),
            SemanticType::Int64 => self.syntax("Value.i64"),
            SemanticType::UInt8 => self.syntax("Value.u8"),
            SemanticType::UInt16 => self.syntax("Value.u16"),
            SemanticType::UInt32 => self.syntax("Value.u32"),
            SemanticType::UInt64 => self.syntax("Value.u64"),
            SemanticType::String => self.syntax("Value.string"),
            SemanticType::Bytes => self.syntax("Value.bytes"),
            SemanticType::Unit => self.library("encUnit"),
            SemanticType::Ordering => self.library("encOrdering"),
            SemanticType::ContractViolation | SemanticType::ReasoningFailure => {
                let bool = self.b.konst("Bool");
                let value_bool = self.syntax("Value.bool");
                let pair = self.library("encPair");
                self.b.apps(pair, &[bool, bool, value_bool, value_bool])
            }
            SemanticType::Option { value } => {
                let alpha = self.lean_type(value)?;
                let inner = self.encoder(value)?;
                let option = self.library("encOption");
                self.b.apps(option, &[alpha, inner])
            }
            SemanticType::Result { ok, error } => {
                let epsilon = self.lean_type(error)?;
                let alpha = self.lean_type(ok)?;
                let error = self.encoder(error)?;
                let ok = self.encoder(ok)?;
                let except = self.library("encExcept");
                self.b.apps(except, &[epsilon, alpha, error, ok])
            }
            SemanticType::List { element } | SemanticType::Set { element } => {
                let alpha = self.lean_type(element)?;
                let inner = self.encoder(element)?;
                let list = self.library("encList");
                self.b.apps(list, &[alpha, inner])
            }
            SemanticType::Product { left, right } => {
                let alpha = self.lean_type(left)?;
                let beta = self.lean_type(right)?;
                let left = self.encoder(left)?;
                let right = self.encoder(right)?;
                let pair = self.library("encPair");
                self.b.apps(pair, &[alpha, beta, left, right])
            }
            SemanticType::Map { key, value } => {
                let entry = SemanticType::Product {
                    left: key.clone(),
                    right: value.clone(),
                };
                let alpha = self.lean_type(&entry)?;
                let inner = self.encoder(&entry)?;
                let list = self.library("encList");
                self.b.apps(list, &[alpha, inner])
            }
            SemanticType::Named {
                member: _,
                arguments: _,
            } => {
                let index = self.adt_index(ty)?;
                self.of_a(&format!("__enc_{index}"))
            }
            SemanticType::Function {
                parameters: _,
                result: _,
            }
            | SemanticType::Type
            | SemanticType::Prop
            | SemanticType::Parameter { name: _ } => {
                return Err(format!(
                    "`{}` has no runtime encoding",
                    self.gen.source.type_text(ty)
                ));
            }
        })
    }

    /// The encoder of a container nested in a recursive group: certificate
    /// A's auxiliary.
    fn nested_encoder(&mut self, nested: &Nested) -> Result<Term, String> {
        let n = nested.index;
        Ok(match &nested.ty {
            SemanticType::List { element }
            | SemanticType::Set { element }
            | SemanticType::Map {
                key: _,
                value: element,
            } => {
                let element = element_type(&nested.ty, element);
                let alpha = self.lean_type(&element)?;
                let enc = self.library("ListEnc.enc");
                let aux = self.of_a(&format!("__L_{n}"));
                self.b.apps(enc, &[alpha, aux])
            }
            SemanticType::ContractViolation | SemanticType::ReasoningFailure => {
                let bool = self.b.konst("Bool");
                let alpha = self.b.pair_ty(bool, bool);
                let enc = self.library("ListEnc.enc");
                let aux = self.of_a(&format!("__L_{n}"));
                self.b.apps(enc, &[alpha, aux])
            }
            SemanticType::Option { value } => {
                let alpha = self.lean_type(value)?;
                let enc = self.library("OptEnc.enc");
                let aux = self.of_a(&format!("__O_{n}"));
                self.b.apps(enc, &[alpha, aux])
            }
            SemanticType::Result { ok: _, error: _ }
            | SemanticType::Product { left: _, right: _ }
            | SemanticType::Named {
                member: _,
                arguments: _,
            }
            | SemanticType::Function {
                parameters: _,
                result: _,
            }
            | SemanticType::Type
            | SemanticType::Prop
            | SemanticType::Parameter { name: _ }
            | SemanticType::Nat
            | SemanticType::Bool
            | SemanticType::Unit
            | SemanticType::Int
            | SemanticType::Int8
            | SemanticType::Int16
            | SemanticType::Int32
            | SemanticType::Int64
            | SemanticType::UInt8
            | SemanticType::UInt16
            | SemanticType::UInt32
            | SemanticType::UInt64
            | SemanticType::String
            | SemanticType::Bytes
            | SemanticType::Ordering => self.of_a(&format!("__aux_{n}")),
        })
    }

    /// The ADT index of a document type in the layout.
    fn adt_index(&self, ty: &SemanticType) -> Result<u64, String> {
        let text = self.gen.source.type_text(ty);
        self.gen
            .adts
            .get(&text)
            .copied()
            .ok_or_else(|| format!("`{text}` has no ADT in the layout"))
    }

    /// `encoder ty` applied to `value`.
    fn encode(&mut self, ty: &SemanticType, value: Term) -> Result<Term, String> {
        let encoder = self.encoder(ty)?;
        Ok(self.b.app(encoder, value))
    }

    // --- Well-typedness proofs --------------------------------------------

    /// The proof that the encoding of `arg : ty` is well typed at the
    /// encoded target type, as [`Gen::wt_apply`] writes it.
    fn wt_apply(&mut self, ty: &SemanticType, arg: Term) -> Result<Term, String> {
        if let Some(nested) = self.gen.nested.get(&self.gen.source.type_text(ty)) {
            let n = nested.index;
            return match &nested.ty {
                SemanticType::List { element: _ }
                | SemanticType::Set { element: _ }
                | SemanticType::Map { key: _, value: _ }
                | SemanticType::ContractViolation
                | SemanticType::ReasoningFailure => {
                    let items = self.of_a(&format!("__items_{n}"));
                    let items = self.b.app(items, arg);
                    let element = super::element_of(&nested.ty)?;
                    let t = self.encoded_ty(&element)?;
                    let wt_items = self.own(&format!("__wtItems_{n}"));
                    let h = self.b.app(wt_items, arg);
                    let (p, c, a) = (self.p, self.c, self.a);
                    self.rust_lemma(
                        "wt_list",
                        &[
                            Arg::Term(p),
                            Arg::Term(c),
                            Arg::Term(a),
                            Arg::Term(items),
                            Arg::Term(t),
                            Arg::Term(h),
                        ],
                    )
                }
                SemanticType::Option { value: _ }
                | SemanticType::Result { ok: _, error: _ }
                | SemanticType::Product { left: _, right: _ }
                | SemanticType::Named {
                    member: _,
                    arguments: _,
                }
                | SemanticType::Function {
                    parameters: _,
                    result: _,
                }
                | SemanticType::Type
                | SemanticType::Prop
                | SemanticType::Parameter { name: _ }
                | SemanticType::Nat
                | SemanticType::Bool
                | SemanticType::Unit
                | SemanticType::Int
                | SemanticType::Int8
                | SemanticType::Int16
                | SemanticType::Int32
                | SemanticType::Int64
                | SemanticType::UInt8
                | SemanticType::UInt16
                | SemanticType::UInt32
                | SemanticType::UInt64
                | SemanticType::String
                | SemanticType::Bytes
                | SemanticType::Ordering => {
                    let aux = self.own(&format!("__wtAux_{n}"));
                    Ok(self.b.app(aux, arg))
                }
            };
        }
        let (p, c, a) = (self.p, self.c, self.a);
        let scalar = |this: &mut Self, lemma: &str| {
            this.rust_lemma(
                lemma,
                &[Arg::Term(p), Arg::Term(c), Arg::Term(a), Arg::Term(arg)],
            )
        };
        match ty {
            SemanticType::Nat => scalar(self, "wt_nat"),
            SemanticType::Int => scalar(self, "wt_int"),
            SemanticType::Bool => scalar(self, "wt_bool"),
            SemanticType::String => scalar(self, "wt_string"),
            SemanticType::Bytes => scalar(self, "wt_bytes"),
            SemanticType::Unit => scalar(self, "wt_encUnit"),
            SemanticType::Ordering => scalar(self, "wt_encOrdering"),
            SemanticType::UInt8 => scalar(self, "wt_u8"),
            SemanticType::UInt16 => scalar(self, "wt_u16"),
            SemanticType::UInt32 => scalar(self, "wt_u32"),
            SemanticType::UInt64 => scalar(self, "wt_u64"),
            SemanticType::Int8 => scalar(self, "wt_i8"),
            SemanticType::Int16 => scalar(self, "wt_i16"),
            SemanticType::Int32 => scalar(self, "wt_i32"),
            SemanticType::Int64 => scalar(self, "wt_i64"),
            SemanticType::Option { value } => {
                let alpha = self.lean_type(value)?;
                let e = self.encoder(value)?;
                let t = self.encoded_ty(value)?;
                let h = self.wt_function(value)?;
                self.rust_lemma(
                    "wt_encOption",
                    &[
                        Arg::Term(p),
                        Arg::Term(c),
                        Arg::Term(a),
                        Arg::Term(alpha),
                        Arg::Term(e),
                        Arg::Term(t),
                        Arg::Term(h),
                        Arg::Term(arg),
                    ],
                )
            }
            SemanticType::List { element } | SemanticType::Set { element } => {
                self.wt_list(element, arg)
            }
            SemanticType::Map { key, value } => {
                let entry = SemanticType::Product {
                    left: key.clone(),
                    right: value.clone(),
                };
                self.wt_list(&entry, arg)
            }
            SemanticType::Product { left, right } => self.wt_pair(left, right, arg),
            SemanticType::ContractViolation | SemanticType::ReasoningFailure => {
                self.wt_pair(&SemanticType::Bool, &SemanticType::Bool, arg)
            }
            SemanticType::Result { ok, error } => {
                let epsilon = self.lean_type(error)?;
                let alpha = self.lean_type(ok)?;
                let ee = self.encoder(error)?;
                let ea = self.encoder(ok)?;
                let s = self.encoded_ty(ok)?;
                let t = self.encoded_ty(error)?;
                let he = self.wt_function(error)?;
                let ha = self.wt_function(ok)?;
                self.rust_lemma(
                    "wt_encExcept",
                    &[
                        Arg::Term(p),
                        Arg::Term(c),
                        Arg::Term(a),
                        Arg::Term(epsilon),
                        Arg::Term(alpha),
                        Arg::Term(ee),
                        Arg::Term(ea),
                        Arg::Term(s),
                        Arg::Term(t),
                        Arg::Term(he),
                        Arg::Term(ha),
                        Arg::Term(arg),
                    ],
                )
            }
            SemanticType::Named {
                member: _,
                arguments: _,
            } => {
                let index = self.adt_index(ty)?;
                let wt = self.own(&format!("__wt_{index}"));
                Ok(self.b.app(wt, arg))
            }
            SemanticType::Function {
                parameters: _,
                result: _,
            }
            | SemanticType::Type
            | SemanticType::Prop
            | SemanticType::Parameter { name: _ } => Err(format!(
                "`{}` crosses no root boundary",
                self.gen.source.type_text(ty)
            )),
        }
    }

    fn wt_list(&mut self, element: &SemanticType, arg: Term) -> Result<Term, String> {
        let (p, c, a) = (self.p, self.c, self.a);
        let alpha = self.lean_type(element)?;
        let e = self.encoder(element)?;
        let t = self.encoded_ty(element)?;
        let h = self.wt_function(element)?;
        self.rust_lemma(
            "wt_encList",
            &[
                Arg::Term(p),
                Arg::Term(c),
                Arg::Term(a),
                Arg::Term(alpha),
                Arg::Term(e),
                Arg::Term(t),
                Arg::Term(h),
                Arg::Term(arg),
            ],
        )
    }

    fn wt_pair(
        &mut self,
        left: &SemanticType,
        right: &SemanticType,
        arg: Term,
    ) -> Result<Term, String> {
        let (p, c, a) = (self.p, self.c, self.a);
        let alpha = self.lean_type(left)?;
        let beta = self.lean_type(right)?;
        let ea = self.encoder(left)?;
        let eb = self.encoder(right)?;
        let s = self.encoded_ty(left)?;
        let t = self.encoded_ty(right)?;
        let ha = self.wt_function(left)?;
        let hb = self.wt_function(right)?;
        self.rust_lemma(
            "wt_encPair",
            &[
                Arg::Term(p),
                Arg::Term(c),
                Arg::Term(a),
                Arg::Term(alpha),
                Arg::Term(beta),
                Arg::Term(ea),
                Arg::Term(eb),
                Arg::Term(s),
                Arg::Term(t),
                Arg::Term(ha),
                Arg::Term(hb),
                Arg::Term(arg),
            ],
        )
    }

    /// `fun (x : ty) => wt_apply ty x`: the well-typedness proof of every
    /// value of a type, which the container lemmas take.
    fn wt_function(&mut self, ty: &SemanticType) -> Result<Term, String> {
        let domain = self.lean_type(ty)?;
        let x = self.b.bvar(0);
        let body = self.wt_apply(ty, x)?;
        Ok(self.b.lam("__x", CoreBinderInfo::Explicit, domain, body))
    }

    /// `WTL p c A [v₁, …] [t₁, …]` from the proofs `WT p c A vᵢ tᵢ`: the
    /// `wtl_cons` chain the text writes, with the values and types each cons
    /// states.
    fn wtl(&mut self, items: &[(Term, Term, Term)]) -> Result<Term, String> {
        let (p, c, a) = (self.p, self.c, self.a);
        let value_ty = self.syntax("Value");
        let ty_ty = self.syntax("Ty");
        let mut proof = self.rust_lemma("wtl_nil", &[Arg::Term(p), Arg::Term(c), Arg::Term(a)])?;
        let mut values: Vec<Term> = Vec::new();
        let mut types: Vec<Term> = Vec::new();
        for &(value, ty, h) in items.iter().rev() {
            let vs = self.b.list(value_ty, &values);
            let ts = self.b.list(ty_ty, &types);
            proof = self.rust_lemma(
                "wtl_cons",
                &[
                    Arg::Term(p),
                    Arg::Term(c),
                    Arg::Term(a),
                    Arg::Term(value),
                    Arg::Term(ty),
                    Arg::Term(vs),
                    Arg::Term(ts),
                    Arg::Term(h),
                    Arg::Term(proof),
                ],
            )?;
            values.insert(0, value);
            types.insert(0, ty);
        }
        Ok(proof)
    }

    /// `WT p c A value (Ty.adt index)`.
    fn wt_statement(&mut self, value: Term, index: u64) -> Term {
        let wt = self.rust("WT");
        let adt = self.syntax("Ty.adt");
        let index = self.b.nat(index);
        let adt = self.b.app(adt, index);
        let (p, c, a) = (self.p, self.c, self.a);
        self.b.apps(wt, &[p, c, a, value, adt])
    }

    /// `wt_adt` for constructor `constructor` of ADT `index`, whose fields
    /// are `fields` (type and the term holding the value), the fields'
    /// well-typedness proven by [`Self::wt_apply`].
    fn wt_adt(
        &mut self,
        index: u64,
        constructor: u64,
        fields: &[(SemanticType, Term)],
    ) -> Result<Term, String> {
        let mut items = Vec::new();
        for (ty, value) in fields {
            let encoded = self.encode(ty, *value)?;
            let target = self.encoded_ty(ty)?;
            let proof = self.wt_apply(ty, *value)?;
            items.push((encoded, target, proof));
        }
        self.wt_adt_items(index, constructor, &items)
    }

    /// The row `__wt_k` of a document type `index` outside every recursive
    /// group: every value of the type encodes to a well-typed value of
    /// `Ty.adt index`, by the projections of a structure or by cases on an
    /// inductive.
    fn wt_row(&mut self, index: u64) -> Result<Row, String> {
        let adt = &self.gen.layout.adts[index as usize];
        let ty = adt.ty.clone();
        let shape = self.gen.source.document(&ty)?;
        let lean = self.lean_type(&ty)?;
        let (member, arguments) = match &ty {
            SemanticType::Named { member, arguments } => (member.clone(), arguments.clone()),
            SemanticType::Type
            | SemanticType::Parameter { name: _ }
            | SemanticType::Nat
            | SemanticType::Bool
            | SemanticType::Prop
            | SemanticType::Unit
            | SemanticType::Int
            | SemanticType::Int8
            | SemanticType::Int16
            | SemanticType::Int32
            | SemanticType::Int64
            | SemanticType::UInt8
            | SemanticType::UInt16
            | SemanticType::UInt32
            | SemanticType::UInt64
            | SemanticType::String
            | SemanticType::Bytes
            | SemanticType::Ordering
            | SemanticType::Option { value: _ }
            | SemanticType::Result { ok: _, error: _ }
            | SemanticType::List { element: _ }
            | SemanticType::Product { left: _, right: _ }
            | SemanticType::Function {
                parameters: _,
                result: _,
            }
            | SemanticType::Map { key: _, value: _ }
            | SemanticType::Set { element: _ }
            | SemanticType::ContractViolation
            | SemanticType::ReasoningFailure => {
                return Err("an ADT of the layout that is not a document type".to_owned());
            }
        };
        let owner = self.document_name(&member)?;
        let binder = if shape.kind == "inductive" {
            "__v"
        } else {
            "__s"
        };
        // The statement: ∀ (v : T), WT p c A (A.__enc_k v) (.adt k).
        let v = self.b.bvar(0);
        let encoder = self.of_a(&format!("__enc_{index}"));
        let encoded = self.b.app(encoder, v);
        let body = self.wt_statement(encoded, index);
        let statement = self.b.pi(binder, CoreBinderInfo::Explicit, lean, body);
        let value = if shape.kind == "inductive" {
            // By cases on the value: T.casesOn params motive v minors.
            let motive = self.b.lam(binder, CoreBinderInfo::Explicit, lean, body);
            let mut params = Vec::new();
            for argument in &arguments {
                params.push(self.lean_type(argument)?);
            }
            let mut minors = Vec::new();
            for (constructor, fields) in shape.fields.iter().enumerate() {
                let count = fields.len();
                let mut field_values = Vec::new();
                for (position, field) in fields.iter().enumerate() {
                    let var = self.b.bvar(count - 1 - position);
                    field_values.push((field.clone(), var));
                }
                let mut minor = self.wt_adt(index, constructor as u64, &field_values)?;
                for (position, field) in fields.iter().enumerate().rev() {
                    let domain = self.lean_type(field)?;
                    // The binder types are closed, so they need no lifting
                    // under the earlier binders.
                    minor = self.b.lam(
                        &format!("__x{position}"),
                        CoreBinderInfo::Explicit,
                        domain,
                        minor,
                    );
                }
                minors.push(minor);
            }
            let cases_on = self.b.konst0(&format!("{owner}.casesOn"));
            let mut args = params;
            args.push(motive);
            args.push(v);
            args.extend(minors);
            let cases = self.b.apps(cases_on, &args);
            self.b.lam(binder, CoreBinderInfo::Explicit, lean, cases)
        } else {
            // A structure: its fields are the projections of the value.
            let mut params = Vec::new();
            for argument in &arguments {
                params.push(self.lean_type(argument)?);
            }
            let mut field_values = Vec::new();
            for (name, field) in shape.field_names.iter().zip(&shape.fields[0]) {
                let projection = self.b.konst(&format!("{owner}.{name}"));
                let projection = self.b.apps(projection, &params);
                let value = self.b.app(projection, v);
                field_values.push((field.clone(), value));
            }
            let proof = self.wt_adt(index, 0, &field_values)?;
            self.b.lam(binder, CoreBinderInfo::Explicit, lean, proof)
        };
        let name = format!("{}.__wt_{index}", self.name);
        let axioms = self.axioms()?;
        Ok(Row {
            name,
            ty: statement,
            value,
            axioms,
        })
    }

    /// The axioms of the row being built: those of the library lemmas it
    /// applies and of the module's own rows it references, read from the
    /// library registry.
    fn axioms(&mut self) -> Result<BTreeSet<String>, String> {
        let mut out = BTreeSet::new();
        for lemma in self.b.take_lemmas() {
            out.extend(preserve::library_axioms(&lemma)?);
        }
        for reference in self.take_references() {
            if let Some(axioms) = self.row_axioms.get(&reference) {
                out.extend(axioms.iter().cloned());
            }
        }
        Ok(out)
    }

    // --- The root ---------------------------------------------------------

    /// The row `root`: the crate's root (or its boundary entry), invoked on
    /// the encoded arguments, realizes certificate A's observation.
    fn root_row(
        &mut self,
        root_module: &str,
        root_name: &str,
        fallible: bool,
    ) -> Result<Row, String> {
        let (parameters, result) = self.gen.source.signature(root_module, root_name, &[])?;
        let count = parameters.len();
        // The parameters are the binders x₀ … xₙ₋₁, then __e_hrep: under all
        // of them, xᵢ is #(n - i) and __e_hrep is #0.
        let mut domains = Vec::new();
        let mut args = Vec::new();
        let mut wt_items = Vec::new();
        let mut applied_names = Vec::new();
        for (position, SemanticParameter { name, r#type }) in parameters.iter().enumerate() {
            domains.push(self.lean_type(r#type)?);
            let var = self.b.bvar(count - position);
            let encoded = self.encode(r#type, var)?;
            let target = self.encoded_ty(r#type)?;
            let proof = self.wt_apply(r#type, var)?;
            args.push(encoded);
            wt_items.push((encoded, target, proof));
            applied_names.push((name.clone(), var));
        }
        let value_ty = self.syntax("Value");
        let args_list = self.b.list(value_ty, &args);
        let (p, c, a) = (self.p, self.c, self.a);
        let (entry, env) = match self.gen.entry_of() {
            Some((entry, _)) => (entry, self.environment(entry, &args)?),
            None => (0, self.environment(0, &args)?),
        };
        let fallible_term = self
            .b
            .konst(if fallible { "Bool.true" } else { "Bool.false" });
        let vars: Vec<Term> = applied_names.iter().map(|(_, var)| *var).collect();
        // The function the crate's root is, `fn` of `compose`: the entry of
        // the program, read out of the program's function list.
        let function_ty = self.syntax("Function");
        let functions = self.syntax("Program.functions");
        let functions = self.b.app(functions, p);
        let entry_index = self.b.nat(entry);
        let default_function = self.default_function();
        let get_d = self.b.konst0("List.getD");
        let function = self.b.apps(
            get_d,
            &[function_ty, functions, entry_index, default_function],
        );
        // hB : ∀ n, FunSem p c A n f, from certificate B's root theorem.
        let nat = self.b.konst("Nat");
        let b_root = self.b.konst(&format!("{}.root", self.module_b));
        let n = self.b.bvar(0);
        let entry_lifted = self.b.nat(entry);
        let hb_body = self.b.apps(b_root, &[n, entry_lifted]);
        let hb = self.b.lam("__e_n", CoreBinderInfo::Explicit, nat, hb_body);
        let hwt = self.wtl(&wt_items)?;
        let is_entry = self.gen.entry_of().is_some();
        let (observation, fits, outcome, h_a) = if is_entry {
            let denote_entry = self.of_a("denoteEntry");
            let observation = self.b.apps(denote_entry, &vars);
            let entry_fits = self.of_a("entryFits");
            let fits = self.b.apps(entry_fits, &vars);
            let entry_value = self.of_a("entryValue");
            let outcome = self.b.apps(entry_value, &vars);
            let entry_theorem = self.of_a("entry");
            let h_a = self.b.apps(entry_theorem, &vars);
            (observation, fits, outcome, h_a)
        } else {
            let denote = self.of_a("denote");
            let observation = self.b.apps(denote, &vars);
            let fits_0 = self.of_a("__fits_0");
            let fits = self.b.apps(fits_0, &vars);
            let root_lean = self.gen.source.lean_name(
                root_module,
                &MemberRef {
                    module: Some(root_module.to_owned()),
                    name: root_name.to_owned(),
                },
            );
            let root_fn = self.b.konst(&root_lean);
            let applied = self.b.apps(root_fn, &vars);
            let outcome = self.encode(&result, applied)?;
            let a_root = self.of_a("root");
            let h_a = self.b.apps(a_root, &vars);
            (observation, fits, outcome, h_a)
        };
        let hne = self.rust_lemma("rel_ne_stuck", &[Arg::Term(fits), Arg::Term(outcome)])?;
        let composed = self.rust_lemma(
            "compose",
            &[
                Arg::Term(p),
                Arg::Term(c),
                Arg::Term(a),
                Arg::Term(entry_index),
                Arg::Term(function),
                Arg::Term(args_list),
                Arg::Term(env),
                Arg::Term(fallible_term),
                Arg::Term(observation),
                Arg::Term(hb),
                Arg::Refl,
                Arg::Term(hwt),
                Arg::Refl,
                Arg::Refl,
                Arg::Term(h_a),
                Arg::Term(hne),
            ],
        )?;
        // The statement and, for an entry, the proof that reshapes compose's
        // conclusion by the entry's two equations.
        let r_out = self.rust("ROut");
        let realizes = self.rust("RealizesFn");
        let rci = self.rust("RCI");
        let fn_ident = self.rust("fnIdent");
        let ident = self.b.app(fn_ident, entry_index);
        let and = self.b.konst("And");
        let exists = self.b.konst1("Exists");
        let (statement_body, proof) = if is_entry {
            let rci_term = {
                let ro = self.b.bvar(0);
                let lifted_args = self.b.lift(args_list, 1, 0);
                self.b.apps(rci, &[c, ident, lifted_args, ro])
            };
            let accepts = self.of_a("accepts");
            let accepted = self.b.apps(accepts, &vars);
            let some_obs = self.library("someObs");
            let denote = self.of_a("denote");
            let denoted = self.b.apps(denote, &vars);
            let accepted_obs = self.b.app(some_obs, denoted);
            let obs_value = self.library("Obs.value");
            let none = self.syntax("Value.none");
            let refused_obs = self.b.app(obs_value, none);
            let not = self.b.konst("Not");
            let refused = self.b.app(not, accepted);
            // Under the binder ro (#0): the two implications, whose bodies
            // are under one more anonymous binder.
            let body = {
                let ro = self.b.bvar(0);
                let accepted1 = self.b.lift(accepted, 1, 0);
                let accepted_obs1 = self.b.lift(accepted_obs, 1, 0);
                let refused1 = self.b.lift(refused, 1, 0);
                let refused_obs1 = self.b.lift(refused_obs, 1, 0);
                let realizes_accepted = {
                    let ro1 = self.b.bvar(1);
                    let f = fallible_term;
                    let obs = self.b.lift(accepted_obs1, 1, 0);
                    self.b.apps(realizes, &[f, obs, ro1])
                };
                let implies_accepted =
                    self.b
                        .pi("", CoreBinderInfo::Explicit, accepted1, realizes_accepted);
                let realizes_refused = {
                    let ro1 = self.b.bvar(1);
                    let f = fallible_term;
                    let obs = self.b.lift(refused_obs1, 1, 0);
                    self.b.apps(realizes, &[f, obs, ro1])
                };
                let implies_refused =
                    self.b
                        .pi("", CoreBinderInfo::Explicit, refused1, realizes_refused);
                let tail = self.b.apps(and, &[implies_accepted, implies_refused]);
                let _ = ro;
                self.b.apps(and, &[rci_term, tail])
            };
            let predicate = self.b.lam("__e_ro", CoreBinderInfo::Explicit, r_out, body);
            let statement = self.b.apps(exists, &[r_out, predicate]);
            // The proof: from compose's ⟨ro, hr, hc⟩ build ⟨ro, hc, _, _⟩,
            // each implication rewriting hr by the entry's equation.
            let compose_predicate = {
                let ro = self.b.bvar(0);
                let lifted_args = self.b.lift(args_list, 1, 0);
                let realizes_obs = {
                    let obs = self.b.lift(observation, 1, 0);
                    self.b.apps(realizes, &[fallible_term, obs, ro])
                };
                let rci_ro = self.b.apps(rci, &[c, ident, lifted_args, ro]);
                let conj = self.b.apps(and, &[realizes_obs, rci_ro]);
                self.b.lam("__e_ro", CoreBinderInfo::Explicit, r_out, conj)
            };
            let proof = {
                // fun (ro : ROut) (hand : RealizesFn F obs ro ∧ RCI …) => …
                let ro = self.b.bvar(1);
                let hand = self.b.bvar(0);
                let obs2 = self.b.lift(observation, 2, 0);
                let args2 = self.b.lift(args_list, 2, 0);
                let realizes_obs = self.b.apps(realizes, &[fallible_term, obs2, ro]);
                let rci_ro = self.b.apps(rci, &[c, ident, args2, ro]);
                let and_left = self.b.konst("And.left");
                let and_right = self.b.konst("And.right");
                let hr = self.b.apps(and_left, &[realizes_obs, rci_ro, hand]);
                let hc = self.b.apps(and_right, &[realizes_obs, rci_ro, hand]);
                let obs_ty = self.library("Obs");
                let motive = {
                    // Under o: hand is #1 and ro is #2.
                    let o = self.b.bvar(0);
                    let ro2 = self.b.bvar(2);
                    let body = self.b.apps(realizes, &[fallible_term, o, ro2]);
                    self.b.lam("__e_o", CoreBinderInfo::Explicit, obs_ty, body)
                };
                let eq_subst = self.b.konst1("Eq.subst");
                let accepted2 = self.b.lift(accepted, 2, 0);
                let accepted_obs2 = self.b.lift(accepted_obs, 2, 0);
                let refused2 = self.b.lift(refused, 2, 0);
                let refused_obs2 = self.b.lift(refused_obs, 2, 0);
                let vars3: Vec<Term> = vars.iter().map(|&var| self.b.lift(var, 3, 0)).collect();
                let accepted_proof = {
                    let h = self.b.bvar(0);
                    let entry_accepts = self.of_a("entry_accepts");
                    let mut equation_args = vars3.clone();
                    equation_args.push(h);
                    let equation = self.b.apps(entry_accepts, &equation_args);
                    let obs3 = self.b.lift(observation, 3, 0);
                    let target = self.b.lift(accepted_obs, 3, 0);
                    let hr3 = self.b.lift(hr, 1, 0);
                    let motive3 = self.b.lift(motive, 1, 0);
                    let rewritten = self
                        .b
                        .apps(eq_subst, &[obs_ty, motive3, obs3, target, equation, hr3]);
                    self.b
                        .lam("__e_h", CoreBinderInfo::Explicit, accepted2, rewritten)
                };
                let refused_proof = {
                    let h = self.b.bvar(0);
                    let entry_refuses = self.of_a("entry_refuses");
                    let mut equation_args = vars3.clone();
                    equation_args.push(h);
                    let equation = self.b.apps(entry_refuses, &equation_args);
                    let obs3 = self.b.lift(observation, 3, 0);
                    let target = self.b.lift(refused_obs, 3, 0);
                    let hr3 = self.b.lift(hr, 1, 0);
                    let motive3 = self.b.lift(motive, 1, 0);
                    let rewritten = self
                        .b
                        .apps(eq_subst, &[obs_ty, motive3, obs3, target, equation, hr3]);
                    self.b
                        .lam("__e_h", CoreBinderInfo::Explicit, refused2, rewritten)
                };
                let and_intro = self.b.konst("And.intro");
                let realizes_accepted2 = self.b.apps(realizes, &[fallible_term, accepted_obs2, ro]);
                let realizes_refused2 = self.b.apps(realizes, &[fallible_term, refused_obs2, ro]);
                let implies_accepted2 = {
                    let lifted = self.b.lift(realizes_accepted2, 1, 0);
                    self.b.pi("", CoreBinderInfo::Explicit, accepted2, lifted)
                };
                let implies_refused2 = {
                    let lifted = self.b.lift(realizes_refused2, 1, 0);
                    self.b.pi("", CoreBinderInfo::Explicit, refused2, lifted)
                };
                let tail_ty = self.b.apps(and, &[implies_accepted2, implies_refused2]);
                let tail = self.b.apps(
                    and_intro,
                    &[
                        implies_accepted2,
                        implies_refused2,
                        accepted_proof,
                        refused_proof,
                    ],
                );
                let witness = self.b.apps(and_intro, &[rci_ro, tail_ty, hc, tail]);
                let exists_intro = self.b.konst1("Exists.intro");
                let predicate2 = self.b.lift(predicate, 2, 0);
                let intro = self.b.apps(exists_intro, &[r_out, predicate2, ro, witness]);
                let hand_ty = self.b.apps(and, &[realizes_obs, rci_ro]);
                let hand_ty = {
                    // hand's type is stated under ro only.
                    let ro1 = self.b.bvar(0);
                    let obs1 = self.b.lift(observation, 1, 0);
                    let args1 = self.b.lift(args_list, 1, 0);
                    let realizes1 = self.b.apps(realizes, &[fallible_term, obs1, ro1]);
                    let rci1 = self.b.apps(rci, &[c, ident, args1, ro1]);
                    let _ = hand_ty;
                    self.b.apps(and, &[realizes1, rci1])
                };
                let inner = self
                    .b
                    .lam("__e_hand", CoreBinderInfo::Explicit, hand_ty, intro);
                let elim_fn = self.b.lam("__e_ro", CoreBinderInfo::Explicit, r_out, inner);
                let exists_elim = self.b.konst1("Exists.elim");
                self.b.apps(
                    exists_elim,
                    &[r_out, compose_predicate, statement, composed, elim_fn],
                )
            };
            (statement, proof)
        } else {
            let body = {
                let ro = self.b.bvar(0);
                let obs1 = self.b.lift(observation, 1, 0);
                let args1 = self.b.lift(args_list, 1, 0);
                let realizes_obs = self.b.apps(realizes, &[fallible_term, obs1, ro]);
                let rci_ro = self.b.apps(rci, &[c, ident, args1, ro]);
                self.b.apps(and, &[realizes_obs, rci_ro])
            };
            let predicate = self.b.lam("__e_ro", CoreBinderInfo::Explicit, r_out, body);
            let statement = self.b.apps(exists, &[r_out, predicate]);
            (statement, composed)
        };
        // Close the binders: the parameters, then __e_hrep, whose type is
        // stated under the parameters alone, one binder above the body.
        let representable = self.rust("RepresentableL");
        let mut outer_args = Vec::new();
        for (position, SemanticParameter { name: _, r#type }) in parameters.iter().enumerate() {
            let var = self.b.bvar(count - 1 - position);
            outer_args.push(self.encode(r#type, var)?);
        }
        let outer_args = self.b.list(value_ty, &outer_args);
        let hrep_ty = self.b.app(representable, outer_args);
        let mut statement = self.b.pi(
            "__e_hrep",
            CoreBinderInfo::Explicit,
            hrep_ty,
            statement_body,
        );
        let mut value = self
            .b
            .lam("__e_hrep", CoreBinderInfo::Explicit, hrep_ty, proof);
        for (position, SemanticParameter { name, r#type: _ }) in parameters.iter().enumerate().rev()
        {
            let domain = domains[position];
            let binder = binder_name(name);
            statement = self
                .b
                .pi(&binder, CoreBinderInfo::Explicit, domain, statement);
            value = self.b.lam(&binder, CoreBinderInfo::Explicit, domain, value);
        }
        let mut axioms = self.axioms()?;
        // Certificates A and B are audited to exactly the certificate axioms,
        // which the composition therefore carries.
        axioms.extend(CERTIFICATE_AXIOMS.iter().map(|axiom| (*axiom).to_owned()));
        Ok(Row {
            name: format!("{}.root", self.name),
            ty: statement,
            value,
            axioms,
        })
    }

    /// The environment `bindAll parameters args []` yields: the parameters
    /// bound to the arguments, the last binding first.
    fn environment(&mut self, function: u64, args: &[Term]) -> Result<Term, String> {
        let parameters = &self
            .gen
            .program
            .functions
            .get(function as usize)
            .ok_or_else(|| format!("the program has no function {function}"))?
            .parameters;
        if parameters.len() != args.len() {
            return Err(format!(
                "function {function} takes {} parameters, the root {} arguments",
                parameters.len(),
                args.len()
            ));
        }
        let nat = self.b.konst("Nat");
        let value_ty = self.syntax("Value");
        let entry_ty = self.b.pair_ty(nat, value_ty);
        let mut bindings = Vec::new();
        for (parameter, arg) in parameters.iter().zip(args).rev() {
            let name = self.b.nat(*parameter);
            bindings.push(self.b.pair(nat, value_ty, name, *arg));
        }
        Ok(self.b.list(entry_ty, &bindings))
    }

    /// A function of the target calculus, as `List.getD`'s fallback: never
    /// read, since the entry is in the list.
    fn default_function(&mut self) -> Term {
        let nat = self.b.konst("Nat");
        let ty_ty = self.syntax("Ty");
        let parameters = self.b.list(nat, &[]);
        let types = self.b.list(ty_ty, &[]);
        let unit_ty = self.syntax("Ty.unit");
        let unit_value = self.syntax("Value.unit");
        let value_expr = self.syntax("Expr.value");
        let body = self.b.apps(value_expr, &[unit_ty, unit_value]);
        let mk = self.syntax("Function.mk");
        self.b.apps(mk, &[parameters, types, unit_ty, body])
    }

    /// The module: the well-typedness rows in dependency order, then `root`.
    fn module(
        mut self,
        root_module: &str,
        root_name: &str,
        fallible: bool,
    ) -> Result<CoreModule, String> {
        let graph = self.gen.adt_graph()?;
        let mut rows = Vec::new();
        for component in components(&graph) {
            let recursive = component.len() > 1
                || component.iter().any(|index| {
                    self.gen
                        .nested
                        .values()
                        .any(|nested| nested.group == *index)
                        || graph.get(index).is_some_and(|edges| edges.contains(index))
                });
            let built = if recursive {
                self.group_rows(&component)?
            } else {
                let mut built = Vec::new();
                for index in component {
                    built.push(self.wt_row(index)?);
                }
                built
            };
            for row in built {
                self.row_axioms.insert(row.name.clone(), row.axioms.clone());
                rows.push(row);
            }
        }
        rows.push(self.root_row(root_module, root_name, fallible)?);
        // Every row is closed: a loose variable is a generator error, named
        // here rather than by the kernel.
        for row in &rows {
            if self.b.has_loose(row.ty, 0) || self.b.has_loose(row.value, 0) {
                return Err(format!(
                    "the row `{}` of certificate E has a loose bound variable",
                    row.name
                ));
            }
        }
        let declarations = rows
            .into_iter()
            .map(|row| theorem_row(row.name, row.ty, row.value, &row.axioms))
            .collect();
        let module = CoreModule {
            spec: CORE_SPEC_2.to_owned(),
            imports: vec![
                self.module_a.clone(),
                self.module_b.clone(),
                COMPOSE_MODULE.to_owned(),
            ],
            nodes: self.b.finish(),
            proof_nodes: Vec::new(),
            declarations,
        };
        module.validate(crate::LANGUAGE_1_2)?;
        Ok(module)
    }
}

/// A container the kernel nests in a recursive group of document types: an
/// auxiliary inductive type of the group's recursors, with its own motive.
pub(super) struct Aux {
    /// The container as the source writes it, which certificate A's nested
    /// auxiliary encoder is registered under.
    pub(super) original: SemanticType,
    /// The container as Lean sees it: a set as a list, a map as a list of
    /// pairs.
    pub(super) canonical: SemanticType,
    pub(super) kind: AuxKind,
    /// Certificate A's auxiliary for the container (`__items_n`, `__aux_n`).
    pub(super) nested_index: u64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum AuxKind {
    List,
    Option,
    Prod,
    Except,
}

/// Where a type stands among the motives of a group's recursors.
#[derive(Clone, Copy)]
enum Position {
    Block(usize),
    Aux(usize),
}

/// A recursive group: its document types in Lean's declaration order (the
/// order of the mutual block) and the containers nested in them, in the
/// order the kernel creates their auxiliary types: every nested container of
/// the block's constructors, then those found inside the auxiliaries, each in
/// turn.
struct Group {
    block: Vec<u64>,
    aux: Vec<Aux>,
}

/// A type as Lean's kernel sees it: a set is a list, a map a list of pairs,
/// a contract violation or reasoning failure a pair of Booleans.
pub(super) fn canonical(ty: &SemanticType) -> SemanticType {
    match ty {
        SemanticType::Set { element } | SemanticType::List { element } => SemanticType::List {
            element: Box::new(canonical(element)),
        },
        SemanticType::Map { key, value } => SemanticType::List {
            element: Box::new(SemanticType::Product {
                left: Box::new(canonical(key)),
                right: Box::new(canonical(value)),
            }),
        },
        SemanticType::ContractViolation | SemanticType::ReasoningFailure => SemanticType::Product {
            left: Box::new(SemanticType::Bool),
            right: Box::new(SemanticType::Bool),
        },
        SemanticType::Option { value } => SemanticType::Option {
            value: Box::new(canonical(value)),
        },
        SemanticType::Product { left, right } => SemanticType::Product {
            left: Box::new(canonical(left)),
            right: Box::new(canonical(right)),
        },
        SemanticType::Result { ok, error } => SemanticType::Result {
            ok: Box::new(canonical(ok)),
            error: Box::new(canonical(error)),
        },
        SemanticType::Named { member, arguments } => SemanticType::Named {
            member: member.clone(),
            arguments: arguments.iter().map(canonical).collect(),
        },
        SemanticType::Function {
            parameters: _,
            result: _,
        }
        | SemanticType::Type
        | SemanticType::Prop
        | SemanticType::Parameter { name: _ }
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Unit
        | SemanticType::Int
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::String
        | SemanticType::Bytes
        | SemanticType::Ordering => ty.clone(),
    }
}

/// One field of a constructor as a minor premise sees it: its type, the
/// variable holding it, and its induction hypothesis when it is recursive.
struct Field {
    ty: SemanticType,
    var: Term,
    hypothesis: Option<Term>,
}

impl Composer<'_, '_> {
    /// The recursive group of one strongly connected component of the ADT
    /// graph.
    fn group(&self, component: &[u64]) -> Result<Group, String> {
        let mut ordered = Vec::new();
        for &index in component {
            let ty = &self.gen.layout.adts[index as usize].ty;
            let member = named_member(ty)?;
            let module = member.module.clone().ok_or("an unanchored document type")?;
            let position = self
                .gen
                .source
                .declaration_position(&module, &member.name)
                .ok_or_else(|| format!("`{}` is not declared in `{module}`", member.name))?;
            ordered.push((position, index));
        }
        ordered.sort_unstable();
        let block: Vec<u64> = ordered.into_iter().map(|(_, index)| index).collect();
        let block_keys: BTreeSet<String> = block
            .iter()
            .map(|&index| self.key(&self.gen.layout.adts[index as usize].ty))
            .collect();
        let mut aux: Vec<Aux> = Vec::new();
        let mut seen: BTreeSet<String> = BTreeSet::new();
        for &index in &block {
            let shape = self
                .gen
                .source
                .document(&self.gen.layout.adts[index as usize].ty)?;
            for fields in &shape.fields {
                for field in fields {
                    self.register_aux(field, &block_keys, &mut aux, &mut seen)?;
                }
            }
        }
        let mut next = 0;
        while next < aux.len() {
            for field in self.aux_fields(&aux[next])? {
                self.register_aux(&field, &block_keys, &mut aux, &mut seen)?;
            }
            next += 1;
        }
        Ok(Group { block, aux })
    }

    /// The key of a type among a group's positions: its canonical spelling as
    /// Lean sees it.
    fn key(&self, ty: &SemanticType) -> String {
        self.gen.source.type_text(&canonical(ty))
    }

    /// Whether a canonical type mentions a type of the block.
    fn mentions(&self, ty: &SemanticType, block_keys: &BTreeSet<String>) -> bool {
        match ty {
            SemanticType::Named {
                member: _,
                arguments: _,
            } => block_keys.contains(&self.gen.source.type_text(ty)),
            SemanticType::List { element } | SemanticType::Option { value: element } => {
                self.mentions(element, block_keys)
            }
            SemanticType::Product { left, right }
            | SemanticType::Result {
                ok: left,
                error: right,
            } => self.mentions(left, block_keys) || self.mentions(right, block_keys),
            SemanticType::Set { element: _ }
            | SemanticType::Map { key: _, value: _ }
            | SemanticType::ContractViolation
            | SemanticType::ReasoningFailure
            | SemanticType::Function {
                parameters: _,
                result: _,
            }
            | SemanticType::Type
            | SemanticType::Prop
            | SemanticType::Parameter { name: _ }
            | SemanticType::Nat
            | SemanticType::Bool
            | SemanticType::Unit
            | SemanticType::Int
            | SemanticType::Int8
            | SemanticType::Int16
            | SemanticType::Int32
            | SemanticType::Int64
            | SemanticType::UInt8
            | SemanticType::UInt16
            | SemanticType::UInt32
            | SemanticType::UInt64
            | SemanticType::String
            | SemanticType::Bytes
            | SemanticType::Ordering => false,
        }
    }

    /// Register the container `original` as an auxiliary of the group when
    /// it nests a type of the block and is not yet registered.
    fn register_aux(
        &self,
        original: &SemanticType,
        block_keys: &BTreeSet<String>,
        aux: &mut Vec<Aux>,
        seen: &mut BTreeSet<String>,
    ) -> Result<(), String> {
        let canonical = canonical(original);
        if !self.mentions(&canonical, block_keys) {
            return Ok(());
        }
        let kind = match &canonical {
            SemanticType::List { element: _ } => AuxKind::List,
            SemanticType::Option { value: _ } => AuxKind::Option,
            SemanticType::Product { left: _, right: _ } => AuxKind::Prod,
            SemanticType::Result { ok: _, error: _ } => AuxKind::Except,
            SemanticType::Named {
                member: _,
                arguments: _,
            } => return Ok(()),
            SemanticType::Set { element: _ }
            | SemanticType::Map { key: _, value: _ }
            | SemanticType::ContractViolation
            | SemanticType::ReasoningFailure
            | SemanticType::Function {
                parameters: _,
                result: _,
            }
            | SemanticType::Type
            | SemanticType::Prop
            | SemanticType::Parameter { name: _ }
            | SemanticType::Nat
            | SemanticType::Bool
            | SemanticType::Unit
            | SemanticType::Int
            | SemanticType::Int8
            | SemanticType::Int16
            | SemanticType::Int32
            | SemanticType::Int64
            | SemanticType::UInt8
            | SemanticType::UInt16
            | SemanticType::UInt32
            | SemanticType::UInt64
            | SemanticType::String
            | SemanticType::Bytes
            | SemanticType::Ordering => return Ok(()),
        };
        let key = self.gen.source.type_text(&canonical);
        if !seen.insert(key) {
            return Ok(());
        }
        let text = self.gen.source.type_text(original);
        let nested_index = self
            .gen
            .nested
            .get(&text)
            .map(|nested| nested.index)
            .ok_or_else(|| {
                format!("`{text}` is nested in a recursive group, but certificate A has no auxiliary encoder for it")
            })?;
        aux.push(Aux {
            original: original.clone(),
            canonical,
            kind,
            nested_index,
        });
        Ok(())
    }

    /// The field types of an auxiliary's constructors other than itself: the
    /// element of a list, the value of an option, the components of a pair,
    /// the error and the value of a result.
    fn aux_fields(&self, aux: &Aux) -> Result<Vec<SemanticType>, String> {
        let parts = container_parts(&aux.original)?;
        let expected = match aux.kind {
            AuxKind::List | AuxKind::Option => 1,
            AuxKind::Prod | AuxKind::Except => 2,
        };
        if parts.len() != expected {
            return Err(format!(
                "`{}` is an auxiliary of the wrong shape",
                self.gen.source.type_text(&aux.original)
            ));
        }
        Ok(parts)
    }

    /// The position of a type among the group's motives, if it has one.
    fn position(&self, group: &Group, ty: &SemanticType) -> Option<Position> {
        let key = self.key(ty);
        if let Some(index) = group
            .block
            .iter()
            .position(|&index| self.key(&self.gen.layout.adts[index as usize].ty) == key)
        {
            return Some(Position::Block(index));
        }
        group
            .aux
            .iter()
            .position(|aux| self.gen.source.type_text(&aux.canonical) == key)
            .map(Position::Aux)
    }

    /// The body of a motive, over `#0` for the value: the statement each
    /// row of the group proves.
    fn motive_body(&mut self, group: &Group, position: Position) -> Result<Term, String> {
        let v = self.b.bvar(0);
        match position {
            Position::Block(block_position) => {
                let index = group.block[block_position];
                let encoder = self.of_a(&format!("__enc_{index}"));
                let encoded = self.b.app(encoder, v);
                Ok(self.wt_statement(encoded, index))
            }
            Position::Aux(aux_position) => {
                let aux = &group.aux[aux_position];
                let n = aux.nested_index;
                let (p, c, a) = (self.p, self.c, self.a);
                match aux.kind {
                    AuxKind::List => {
                        let element = super::element_of(&aux.original)?;
                        let items = self.of_a(&format!("__items_{n}"));
                        let items = self.b.app(items, v);
                        let t = self.encoded_ty(&element)?;
                        let wt_all = self.rust("WTAll");
                        Ok(self.b.apps(wt_all, &[p, c, a, items, t]))
                    }
                    AuxKind::Option | AuxKind::Prod | AuxKind::Except => {
                        let aux_encoder = self.of_a(&format!("__aux_{n}"));
                        let encoded = self.b.app(aux_encoder, v);
                        let t = self.encoded_ty(&aux.original.clone())?;
                        let wt = self.rust("WT");
                        Ok(self.b.apps(wt, &[p, c, a, encoded, t]))
                    }
                }
            }
        }
    }

    /// The Lean type of the values a position's motive ranges over.
    fn position_type(&mut self, group: &Group, position: Position) -> Result<Term, String> {
        match position {
            Position::Block(block_position) => {
                let ty = self.gen.layout.adts[group.block[block_position] as usize]
                    .ty
                    .clone();
                self.lean_type(&ty)
            }
            Position::Aux(aux_position) => {
                let ty = group.aux[aux_position].canonical.clone();
                self.lean_type(&ty)
            }
        }
    }

    /// The well-typedness proof of a field inside a minor premise: its
    /// induction hypothesis when it is recursive, else the proof of
    /// [`Self::wt_apply`].
    fn wt_field(&mut self, group: &Group, field: &Field) -> Result<Term, String> {
        let Some(hypothesis) = field.hypothesis else {
            return self.wt_apply(&field.ty, field.var);
        };
        match self.position(group, &field.ty) {
            Some(Position::Aux(aux_position)) if group.aux[aux_position].kind == AuxKind::List => {
                let n = group.aux[aux_position].nested_index;
                let element = super::element_of(&group.aux[aux_position].original)?;
                let items = self.of_a(&format!("__items_{n}"));
                let items = self.b.app(items, field.var);
                let t = self.encoded_ty(&element)?;
                let (p, c, a) = (self.p, self.c, self.a);
                self.rust_lemma(
                    "wt_list",
                    &[
                        Arg::Term(p),
                        Arg::Term(c),
                        Arg::Term(a),
                        Arg::Term(items),
                        Arg::Term(t),
                        Arg::Term(hypothesis),
                    ],
                )
            }
            Some(Position::Block(_) | Position::Aux(_)) => Ok(hypothesis),
            None => Err(format!(
                "`{}` has an induction hypothesis but no motive",
                self.gen.source.type_text(&field.ty)
            )),
        }
    }

    /// A minor premise: `fun fields… hypotheses… => body`, the hypotheses
    /// for the recursive fields in field order, `body` built over the fields.
    #[allow(clippy::type_complexity)]
    fn minor(
        &mut self,
        group: &Group,
        fields: &[SemanticType],
        body: &dyn Fn(&mut Self, &[Field]) -> Result<Term, String>,
    ) -> Result<Term, String> {
        let positions: Vec<Option<Position>> = fields
            .iter()
            .map(|field| self.position(group, field))
            .collect();
        let m = fields.len();
        let r = positions
            .iter()
            .filter(|position| position.is_some())
            .count();
        let mut bound = Vec::new();
        let mut k = 0;
        for (j, field) in fields.iter().enumerate() {
            let var = self.b.bvar(m + r - 1 - j);
            let hypothesis = positions[j].map(|_| {
                let term = self.b.bvar(r - 1 - k);
                k += 1;
                term
            });
            bound.push(Field {
                ty: field.clone(),
                var,
                hypothesis,
            });
        }
        let mut term = body(self, &bound)?;
        // The hypotheses, innermost first: the k-th binds at depth m + k,
        // where field j is #(m + k - 1 - j).
        let recursive: Vec<(usize, Position)> = positions
            .iter()
            .enumerate()
            .filter_map(|(j, position)| position.map(|position| (j, position)))
            .collect();
        for (k, (j, position)) in recursive.iter().enumerate().rev() {
            let var = self.b.bvar(m + k - 1 - j);
            let motive = self.motive_body(group, *position)?;
            let domain = self.b.instantiate(motive, var);
            term = self
                .b
                .lam(&format!("__ih{k}"), CoreBinderInfo::Explicit, domain, term);
        }
        for (j, field) in fields.iter().enumerate().rev() {
            let domain = self.lean_type(field)?;
            term = self
                .b
                .lam(&format!("__x{j}"), CoreBinderInfo::Explicit, domain, term);
        }
        Ok(term)
    }

    /// The minor premises of the group's recursors: the block's constructors
    /// in order, then each auxiliary's.
    fn minors(&mut self, group: &Group) -> Result<Vec<Term>, String> {
        let mut out = Vec::new();
        for &index in &group.block {
            let ty = self.gen.layout.adts[index as usize].ty.clone();
            let shape = self.gen.source.document(&ty)?;
            for (constructor, fields) in shape.fields.iter().enumerate() {
                let constructor = constructor as u64;
                let minor = self.minor(group, fields, &|this, bound| {
                    let mut items = Vec::new();
                    for field in bound {
                        let encoded = this.encode(&field.ty, field.var)?;
                        let target = this.encoded_ty(&field.ty)?;
                        let proof = this.wt_field(group, field)?;
                        items.push((encoded, target, proof));
                    }
                    this.wt_adt_items(index, constructor, &items)
                })?;
                out.push(minor);
            }
        }
        for aux_position in 0..group.aux.len() {
            let aux = &group.aux[aux_position];
            let original = aux.original.clone();
            let n = aux.nested_index;
            let (p, c, a) = (self.p, self.c, self.a);
            match aux.kind {
                AuxKind::List => {
                    let element = super::element_of(&original)?;
                    let t = self.encoded_ty(&element)?;
                    out.push(self.rust_lemma(
                        "wtAll_nil",
                        &[Arg::Term(p), Arg::Term(c), Arg::Term(a), Arg::Term(t)],
                    )?);
                    let fields = [element.clone(), original.clone()];
                    out.push(self.minor(group, &fields, &|this, bound| {
                        let head = &bound[0];
                        let tail = &bound[1];
                        let v = this.encode(&head.ty, head.var)?;
                        let items = this.of_a(&format!("__items_{n}"));
                        let vs = this.b.app(items, tail.var);
                        let t = this.encoded_ty(&head.ty)?;
                        let h = this.wt_field(group, head)?;
                        let hs = tail
                            .hypothesis
                            .ok_or("a list auxiliary whose tail has no hypothesis")?;
                        this.rust_lemma(
                            "wtAll_cons",
                            &[
                                Arg::Term(p),
                                Arg::Term(c),
                                Arg::Term(a),
                                Arg::Term(v),
                                Arg::Term(vs),
                                Arg::Term(t),
                                Arg::Term(h),
                                Arg::Term(hs),
                            ],
                        )
                    })?);
                }
                AuxKind::Option => {
                    let inner = container_parts(&original)?
                        .pop()
                        .ok_or("an option auxiliary of a non-option")?;
                    let t = self.encoded_ty(&inner)?;
                    out.push(self.rust_lemma(
                        "wt_noneV",
                        &[Arg::Term(p), Arg::Term(c), Arg::Term(a), Arg::Term(t)],
                    )?);
                    let fields = [inner];
                    out.push(self.minor(group, &fields, &|this, bound| {
                        let val = &bound[0];
                        let v = this.encode(&val.ty, val.var)?;
                        let t = this.encoded_ty(&val.ty)?;
                        let h = this.wt_field(group, val)?;
                        this.rust_lemma(
                            "wt_someV",
                            &[
                                Arg::Term(p),
                                Arg::Term(c),
                                Arg::Term(a),
                                Arg::Term(v),
                                Arg::Term(t),
                                Arg::Term(h),
                            ],
                        )
                    })?);
                }
                AuxKind::Prod => {
                    let fields = self.aux_fields(aux)?;
                    out.push(self.minor(group, &fields, &|this, bound| {
                        let (left, right) = (&bound[0], &bound[1]);
                        let va = this.encode(&left.ty, left.var)?;
                        let vb = this.encode(&right.ty, right.var)?;
                        let s = this.encoded_ty(&left.ty)?;
                        let t = this.encoded_ty(&right.ty)?;
                        let ha = this.wt_field(group, left)?;
                        let hb = this.wt_field(group, right)?;
                        this.rust_lemma(
                            "wt_pairV",
                            &[
                                Arg::Term(p),
                                Arg::Term(c),
                                Arg::Term(a),
                                Arg::Term(va),
                                Arg::Term(vb),
                                Arg::Term(s),
                                Arg::Term(t),
                                Arg::Term(ha),
                                Arg::Term(hb),
                            ],
                        )
                    })?);
                }
                AuxKind::Except => {
                    let mut parts = container_parts(&original)?;
                    let ok = parts.pop().ok_or("a result auxiliary of a non-result")?;
                    let error = parts.pop().ok_or("a result auxiliary of a non-result")?;
                    let s = self.encoded_ty(&ok)?;
                    let t = self.encoded_ty(&error)?;
                    let error_fields = [error.clone()];
                    out.push(self.minor(group, &error_fields, &|this, bound| {
                        let field = &bound[0];
                        let v = this.encode(&field.ty, field.var)?;
                        let h = this.wt_field(group, field)?;
                        this.rust_lemma(
                            "wt_errorV",
                            &[
                                Arg::Term(p),
                                Arg::Term(c),
                                Arg::Term(a),
                                Arg::Term(v),
                                Arg::Term(s),
                                Arg::Term(t),
                                Arg::Term(h),
                            ],
                        )
                    })?);
                    let ok_fields = [ok.clone()];
                    out.push(self.minor(group, &ok_fields, &|this, bound| {
                        let field = &bound[0];
                        let v = this.encode(&field.ty, field.var)?;
                        let h = this.wt_field(group, field)?;
                        this.rust_lemma(
                            "wt_okV",
                            &[
                                Arg::Term(p),
                                Arg::Term(c),
                                Arg::Term(a),
                                Arg::Term(v),
                                Arg::Term(s),
                                Arg::Term(t),
                                Arg::Term(h),
                            ],
                        )
                    })?);
                }
            }
        }
        Ok(out)
    }

    /// `wt_adt` from the encoded fields, their types, and their proofs.
    fn wt_adt_items(
        &mut self,
        index: u64,
        constructor: u64,
        items: &[(Term, Term, Term)],
    ) -> Result<Term, String> {
        let value_ty = self.syntax("Value");
        let ty_ty = self.syntax("Ty");
        let fs: Vec<Term> = items.iter().map(|item| item.0).collect();
        let tys: Vec<Term> = items.iter().map(|item| item.1).collect();
        let fs = self.b.list(value_ty, &fs);
        let tys = self.b.list(ty_ty, &tys);
        let hf = self.wtl(items)?;
        let (p, c, a) = (self.p, self.c, self.a);
        let i = self.b.nat(index);
        let k = self.b.nat(constructor);
        self.rust_lemma(
            "wt_adt",
            &[
                Arg::Term(p),
                Arg::Term(c),
                Arg::Term(a),
                Arg::Term(i),
                Arg::Term(k),
                Arg::Term(fs),
                Arg::Term(tys),
                Arg::Refl,
                Arg::Term(hf),
            ],
        )
    }

    /// The rows of a recursive group: one per document type and one per
    /// nested container, each the group's recursor at the motives and minor
    /// premises the group shares (`T.rec` for a document type, `T.rec_j` of
    /// the block's first type for the `j`-th auxiliary).
    fn group_rows(&mut self, component: &[u64]) -> Result<Vec<Row>, String> {
        let group = self.group(component)?;
        let first = self.gen.layout.adts[group.block[0] as usize].ty.clone();
        let first_member = named_member(&first)?;
        let first_arguments = type_arguments(&first);
        let first_name = self.document_name(&first_member)?;
        let mut params = Vec::new();
        for argument in &first_arguments {
            params.push(self.lean_type(argument)?);
        }
        let mut positions: Vec<Position> = (0..group.block.len()).map(Position::Block).collect();
        positions.extend((0..group.aux.len()).map(Position::Aux));
        let mut motives = Vec::new();
        let mut bodies = Vec::new();
        for &position in &positions {
            let domain = self.position_type(&group, position)?;
            let body = self.motive_body(&group, position)?;
            motives.push(self.b.lam("__v", CoreBinderInfo::Explicit, domain, body));
            bodies.push((domain, body));
        }
        let minors = self.minors(&group)?;
        let axioms = self.axioms()?;
        let mut rows = Vec::new();
        for (ordinal, &position) in positions.iter().enumerate() {
            let (domain, body) = bodies[ordinal];
            let (recursor, name) = match position {
                Position::Block(block_position) => {
                    let index = group.block[block_position];
                    let member = named_member(&self.gen.layout.adts[index as usize].ty)?;
                    let owner = self.document_name(&member)?;
                    (format!("{owner}.rec"), format!("__wt_{index}"))
                }
                Position::Aux(aux_position) => {
                    let aux = &group.aux[aux_position];
                    let name = match aux.kind {
                        AuxKind::List => format!("__wtItems_{}", aux.nested_index),
                        AuxKind::Option | AuxKind::Prod | AuxKind::Except => {
                            format!("__wtAux_{}", aux.nested_index)
                        }
                    };
                    (format!("{first_name}.rec_{}", aux_position + 1), name)
                }
            };
            let recursor = self.b.konst0(&recursor);
            let v = self.b.bvar(0);
            let mut args = params.clone();
            args.extend(motives.iter().copied());
            args.extend(minors.iter().copied());
            args.push(v);
            let applied = self.b.apps(recursor, &args);
            let value = self.b.lam("__v", CoreBinderInfo::Explicit, domain, applied);
            let statement = self.b.pi("__v", CoreBinderInfo::Explicit, domain, body);
            rows.push(Row {
                name: format!("{}.{name}", self.name),
                ty: statement,
                value,
                axioms: axioms.clone(),
            });
        }
        // Every auxiliary certificate A registered for the group has a row,
        // or a proof somewhere else would name a theorem that is not there.
        for nested in self.gen.nested.values() {
            if component.contains(&nested.group)
                && !group.aux.iter().any(|aux| aux.nested_index == nested.index)
            {
                return Err(format!(
                    "`{}` is an auxiliary of the group but the kernel nests no such type",
                    self.gen.source.type_text(&nested.ty)
                ));
            }
        }
        Ok(rows)
    }
}

/// The field types of a container's constructors other than the container
/// itself: the element of a list, set, or map (its entries), the value of an
/// option, the two components of a pair, and the error then the value of a
/// result.
pub(super) fn container_parts(ty: &SemanticType) -> Result<Vec<SemanticType>, String> {
    Ok(match ty {
        SemanticType::List { element } | SemanticType::Set { element } => {
            vec![element.as_ref().clone()]
        }
        SemanticType::Map { key, value } => vec![SemanticType::Product {
            left: key.clone(),
            right: value.clone(),
        }],
        SemanticType::Option { value } => vec![value.as_ref().clone()],
        SemanticType::Product { left, right } => {
            vec![left.as_ref().clone(), right.as_ref().clone()]
        }
        SemanticType::ContractViolation | SemanticType::ReasoningFailure => {
            vec![SemanticType::Bool, SemanticType::Bool]
        }
        SemanticType::Result { ok, error } => vec![error.as_ref().clone(), ok.as_ref().clone()],
        SemanticType::Named {
            member: _,
            arguments: _,
        }
        | SemanticType::Type
        | SemanticType::Prop
        | SemanticType::Parameter { name: _ }
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Unit
        | SemanticType::Int
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::String
        | SemanticType::Bytes
        | SemanticType::Ordering
        | SemanticType::Function {
            parameters: _,
            result: _,
        } => return Err("the constructors of a non-container".to_owned()),
    })
}

/// The type arguments a document type is instantiated at.
pub(super) fn type_arguments(ty: &SemanticType) -> Vec<SemanticType> {
    match ty {
        SemanticType::Named {
            member: _,
            arguments,
        } => arguments.clone(),
        SemanticType::List { element: _ }
        | SemanticType::Set { element: _ }
        | SemanticType::Map { key: _, value: _ }
        | SemanticType::Option { value: _ }
        | SemanticType::Product { left: _, right: _ }
        | SemanticType::Result { ok: _, error: _ }
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure
        | SemanticType::Type
        | SemanticType::Prop
        | SemanticType::Parameter { name: _ }
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Unit
        | SemanticType::Int
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::String
        | SemanticType::Bytes
        | SemanticType::Ordering
        | SemanticType::Function {
            parameters: _,
            result: _,
        } => Vec::new(),
    }
}

/// The member a document type names.
pub(super) fn named_member(ty: &SemanticType) -> Result<MemberRef, String> {
    match ty {
        SemanticType::Named {
            member,
            arguments: _,
        } => Ok(member.clone()),
        SemanticType::List { element: _ }
        | SemanticType::Set { element: _ }
        | SemanticType::Map { key: _, value: _ }
        | SemanticType::Option { value: _ }
        | SemanticType::Product { left: _, right: _ }
        | SemanticType::Result { ok: _, error: _ }
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure
        | SemanticType::Type
        | SemanticType::Prop
        | SemanticType::Parameter { name: _ }
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Unit
        | SemanticType::Int
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::String
        | SemanticType::Bytes
        | SemanticType::Ordering
        | SemanticType::Function {
            parameters: _,
            result: _,
        } => Err("a document type that is not named".to_owned()),
    }
}

/// The binder a parameter is named by in the certificate, as the text
/// generator names it.
fn binder_name(name: &str) -> String {
    match name.strip_prefix("__") {
        Some(rest) if !name.contains('.') => format!("_y{rest}"),
        _ => name.to_owned(),
    }
}

/// The element type of a container nested in a recursive group.
fn element_type(container: &SemanticType, element: &SemanticType) -> SemanticType {
    match container {
        SemanticType::Map { key, value: _ } => SemanticType::Product {
            left: key.clone(),
            right: Box::new(element.clone()),
        },
        SemanticType::List { element: _ }
        | SemanticType::Set { element: _ }
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure
        | SemanticType::Option { value: _ }
        | SemanticType::Result { ok: _, error: _ }
        | SemanticType::Product { left: _, right: _ }
        | SemanticType::Named {
            member: _,
            arguments: _,
        }
        | SemanticType::Function {
            parameters: _,
            result: _,
        }
        | SemanticType::Type
        | SemanticType::Prop
        | SemanticType::Parameter { name: _ }
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Unit
        | SemanticType::Int
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::String
        | SemanticType::Bytes
        | SemanticType::Ordering => element.clone(),
    }
}

/// A theorem row of the module, its axioms pinned exactly.
fn theorem_row(name: String, ty: Term, value: Term, axioms: &BTreeSet<String>) -> CoreDeclaration {
    CoreDeclaration {
        all: Some(vec![name.clone()]),
        name,
        levels: Vec::new(),
        kind: CoreDeclKind::Theorem,
        reducibility: None,
        transparency: CoreTransparency::Semireducible,
        policy: CoreAxiomPolicy::Exact(axioms.iter().cloned().collect()),
        r#type: ty,
        value: Some(value),
        inductive: None,
        class: false,
        instance: None,
        generated: false,
        safety: None,
        compiled: false,
        tags: Vec::new(),
        no_confusion: None,
        matcher: None,
        projection: None,
        simp: None,
        eqn_info: None,
        inline: None,
        sparse: None,
        grind: None,
    }
}

impl Gen<'_> {
    /// Certificate E of the root as a native core module (SPEC.md §17.17).
    pub(super) fn compose_data(
        &self,
        root_module: &str,
        root_name: &str,
        module_a: &str,
        module_b: &str,
        name: &str,
        fallible: bool,
    ) -> Result<CoreModule, String> {
        Composer::new(self, module_a, module_b, name).module(root_module, root_name, fallible)
    }
}
