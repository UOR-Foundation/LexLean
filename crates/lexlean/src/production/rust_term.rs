//! A rendered crate as a Lean term of `LexLeanTarget.RustSyntax.Crate`
//! (SPEC.md §17.17): the closed Rust AST the printer writes as `lib.rs`,
//! construct for construct, without the origins, which carry no meaning.
//! Certificate B states its theorem about this term; that the printer
//! writes the same AST as text is the printer's own obligation (§17.16).

use crate::backend::semantic::string_literal;
use crate::calculus::rust::ast::{
    Block, Callee, CaptureRead, Crate, Ctor, Dispatch, Expr, Ident, ItemDef, Let, Lit, Pat, Type,
};
use crate::calculus::rust::runtime::Item;
use crate::calculus::rust::Profile;
use crate::calculus::{IntKind, OrderingValue};

/// The Lean module of the closed Rust AST.
pub const RUST_SYNTAX: &str = "LexLeanTarget.RustSyntax";

fn list(items: impl IntoIterator<Item = String>) -> String {
    format!("[{}]", items.into_iter().collect::<Vec<_>>().join(", "))
}

fn kind(width: IntKind) -> String {
    format!(".{}", width.name())
}

fn signed(number: i128) -> String {
    if number < 0 {
        format!("({number})")
    } else {
        number.to_string()
    }
}

fn order(value: OrderingValue) -> &'static str {
    match value {
        OrderingValue::Lt => ".less",
        OrderingValue::Eq => ".same",
        OrderingValue::Gt => ".more",
    }
}

/// An identifier as a `RustSyntax.Ident` term.
#[must_use]
pub fn ident(name: &Ident) -> String {
    let generated = |kind: &str, index: u64| format!("(.generated .{kind} {index})");
    match name {
        Ident::Local(n) => generated("binding", *n),
        Ident::Holder(n) => generated("holder", *n),
        Ident::Operand(n) => generated("operand", *n),
        Ident::Callee(n) => generated("callee", *n),
        Ident::Boxed(n) => generated("boxed", *n),
        Ident::Part(n) => generated("part", *n),
        Ident::Capture(n) => generated("capture", *n),
        Ident::Param(n) => generated("param", *n),
        Ident::Function(n) => generated("function", *n),
        Ident::Export(text) => format!("(.exported {})", string_literal(text)),
    }
}

fn ty(written: &Type) -> String {
    let inner = |name: &str, inner: &Type| format!("(.{name} {})", ty(inner));
    match written {
        Type::Unit => ".unit".to_owned(),
        Type::Bool => ".bool".to_owned(),
        Type::Nat => ".nat".to_owned(),
        Type::Int => ".int".to_owned(),
        Type::Fixed(width) => format!("(.fixed {})", kind(*width)),
        Type::Ordering => ".ordering".to_owned(),
        Type::Str => ".str".to_owned(),
        Type::Bytes => ".bytes".to_owned(),
        Type::Option(held) => inner("option", held),
        Type::Result(ok, error) => format!("(.result {} {})", ty(ok), ty(error)),
        Type::List(element) => inner("list", element),
        Type::Pair(left, right) => format!("(.pair {} {})", ty(left), ty(right)),
        Type::Adt(n) => format!("(.adt {n})"),
        Type::Fn(n) => format!("(.fn {n})"),
        Type::Rc(held) => inner("rc", held),
        Type::Fallible(held) => inner("fallible", held),
        Type::Ref(held) => inner("ref", held),
    }
}

fn lit(literal: &Lit) -> String {
    match literal {
        Lit::Unit => ".unit".to_owned(),
        Lit::Bool(flag) => format!("(.bool {flag})"),
        Lit::Nat(number) => format!("(.nat {number})"),
        Lit::Int(number) => format!("(.int {})", signed(i128::from(*number))),
        Lit::Fixed(width, number) => format!("(.fixed {} {})", kind(*width), signed(*number)),
        Lit::Str(text) => format!("(.str {})", string_literal(text)),
        Lit::Bytes(octets) => format!(
            "(.bytes (ByteArray.mk #[{}]))",
            octets
                .iter()
                .map(u8::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Lit::Ordering(value) => format!("(.ordering {})", order(*value)),
    }
}

fn ctor(built: &Ctor) -> String {
    match built {
        Ctor::None(held) => format!("(.none {})", ty(held)),
        Ctor::Some => ".some".to_owned(),
        Ctor::Ok(ok, error) => format!("(.ok {} {})", ty(ok), ty(error)),
        Ctor::Err(ok, error) => format!("(.err {} {})", ty(ok), ty(error)),
        Ctor::Adt { adt, constructor } => format!("(.adt {adt} {constructor})"),
        Ctor::Closure { fn_type, function } => format!("(.closure {fn_type} {function})"),
        Ctor::Cons => ".cons".to_owned(),
        Ctor::Nil(element) => format!("(.nil {})", ty(element)),
    }
}

/// A pattern as a `RustSyntax.Pat` term.
#[must_use]
pub fn pat(pattern: &Pat) -> String {
    match pattern {
        Pat::Wild => ".wild".to_owned(),
        Pat::Bind(name) => format!("(.bind {})", ident(name)),
        Pat::Unit => ".unit".to_owned(),
        Pat::Tuple(parts) => format!("(.tuple {})", list(parts.iter().map(pat))),
        Pat::None => ".none".to_owned(),
        Pat::Some(inner) => format!("(.some {})", pat(inner)),
        Pat::Ok(inner) => format!("(.ok {})", pat(inner)),
        Pat::Err(inner) => format!("(.err {})", pat(inner)),
        Pat::Ordering(value) => format!("(.ordering {})", order(*value)),
        Pat::Adt {
            adt,
            constructor,
            fields,
        } => format!(
            "(.adt {adt} {constructor} {})",
            list(fields.iter().map(pat))
        ),
    }
}

/// A runtime item as a `RustSyntax.Item` term.
#[must_use]
pub fn item(runtime: Item) -> String {
    match runtime {
        Item::NatAdd => ".natAdd".to_owned(),
        Item::NatSub => ".natSub".to_owned(),
        Item::NatMul => ".natMul".to_owned(),
        Item::NatQuot => ".natQuot".to_owned(),
        Item::NatRem => ".natRem".to_owned(),
        Item::NatEq => ".natEq".to_owned(),
        Item::NatLe => ".natLe".to_owned(),
        Item::NatLt => ".natLt".to_owned(),
        Item::NatSucc => ".natSucc".to_owned(),
        Item::IntAdd => ".intAdd".to_owned(),
        Item::IntSub => ".intSub".to_owned(),
        Item::IntMul => ".intMul".to_owned(),
        Item::IntNeg => ".intNeg".to_owned(),
        Item::IntQuot => ".intQuot".to_owned(),
        Item::IntRem => ".intRem".to_owned(),
        Item::BoolNot => ".boolNot".to_owned(),
        Item::BoolAnd => ".boolAnd".to_owned(),
        Item::BoolOr => ".boolOr".to_owned(),
        Item::Equal => ".equal".to_owned(),
        Item::Compare => ".compare".to_owned(),
        Item::CheckedAdd(width) => format!("(.checkedAdd {})", kind(width)),
        Item::CheckedSub(width) => format!("(.checkedSub {})", kind(width)),
        Item::CheckedMul(width) => format!("(.checkedMul {})", kind(width)),
        Item::CheckedQuot(width) => format!("(.checkedQuot {})", kind(width)),
        Item::CheckedNeg(width) => format!("(.checkedNeg {})", kind(width)),
        Item::BitAnd(width) => format!("(.bitAnd {})", kind(width)),
        Item::BitOr(width) => format!("(.bitOr {})", kind(width)),
        Item::BitXor(width) => format!("(.bitXor {})", kind(width)),
        Item::BitNot(width) => format!("(.bitNot {})", kind(width)),
        Item::ShiftLeft(width) => format!("(.shiftLeft {})", kind(width)),
        Item::ShiftRight(width) => format!("(.shiftRight {})", kind(width)),
        Item::Convert(width) => format!("(.convert {})", kind(width)),
        Item::AppendList => ".appendList".to_owned(),
        Item::AppendBytes => ".appendBytes".to_owned(),
        Item::LengthList => ".lengthList".to_owned(),
        Item::LengthBytes => ".lengthBytes".to_owned(),
        Item::LengthString => ".lengthString".to_owned(),
        Item::IndexList => ".indexList".to_owned(),
        Item::IndexBytes => ".indexBytes".to_owned(),
        Item::SliceList => ".sliceList".to_owned(),
        Item::SliceBytes => ".sliceBytes".to_owned(),
        Item::Utf8Encode => ".utf8Encode".to_owned(),
        Item::Utf8Decode => ".utf8Decode".to_owned(),
        Item::CompareBytes => ".compareBytes".to_owned(),
        Item::SplitExact => ".splitExact".to_owned(),
        Item::Join => ".join".to_owned(),
        Item::FormatInt => ".formatInt".to_owned(),
        Item::FormatFixed(width) => format!("(.formatFixed {})", kind(width)),
        Item::ParseInt => ".parseInt".to_owned(),
        Item::ParseFixed(width) => format!("(.parseFixed {})", kind(width)),
    }
}

fn exprs(all: &[Expr]) -> String {
    list(all.iter().map(expr))
}

/// One expression as a `RustSyntax.Expr` term.
#[must_use]
pub fn expr(written: &Expr) -> String {
    let read = |name: &str, held: &Ident| format!("(.{name} {})", ident(held));
    match written {
        Expr::Lit(literal, _) => format!("(.lit {})", lit(literal)),
        Expr::Move(held, _) => read("move", held),
        Expr::Clone(held, _) => read("clone", held),
        Expr::Copy(held, _) => read("copy", held),
        Expr::Deref(held, _) => read("deref", held),
        Expr::Not(inner, _) => format!("(.not {})", expr(inner)),
        Expr::Unbox(held, _) => read("unbox", held),
        Expr::Box(inner, _) => format!("(.box {})", expr(inner)),
        Expr::Call {
            callee,
            args,
            propagate,
            at: _,
        } => {
            let callee = match callee {
                Callee::Function(n) => format!("(.function {n})"),
                Callee::Runtime(runtime) => format!("(.runtime {})", item(*runtime)),
            };
            format!("(.call {callee} {} {propagate})", exprs(args))
        }
        Expr::Apply {
            holder,
            args,
            propagate,
            at: _,
        } => format!("(.apply {} {} {propagate})", ident(holder), exprs(args)),
        Expr::Construct {
            ctor: built,
            args,
            at: _,
        } => {
            format!("(.construct {} {})", ctor(built), exprs(args))
        }
        Expr::Pair(left, right, _) => format!("(.pair {} {})", expr(left), expr(right)),
        Expr::If {
            condition,
            then_branch,
            else_branch,
            at: _,
        } => format!(
            "(.cond {} {} {})",
            expr(condition),
            block(then_branch),
            block(else_branch)
        ),
        Expr::Match {
            scrutinee,
            arms,
            at: _,
        } => format!(
            "(.matchOn {} {})",
            expr(scrutinee),
            list(arms.iter().map(|(pattern, body)| format!(
                "(.mk {} {})",
                pat(pattern),
                block(body)
            )))
        ),
        Expr::Block(inner) => format!("(.block {})", block(inner)),
        Expr::Uncons(held) => read("uncons", held),
        Expr::IsZero(held, _) => read("isZero", held),
        Expr::NonZero(held, _) => read("nonZero", held),
        Expr::Predecessor(held) => read("predecessor", held),
        Expr::Widen(inner, _) => format!("(.widen {})", expr(inner)),
        Expr::Succeed(inner, _) => format!("(.succeed {})", expr(inner)),
    }
}

/// One let as a `RustSyntax.Let` term.
#[must_use]
pub fn binding(written: &Let) -> String {
    let annotated = match &written.ty {
        Some(declared) => format!("(some {})", ty(declared)),
        None => "none".to_owned(),
    };
    format!(
        "(.mk {} {annotated} {})",
        pat(&written.pat),
        expr(&written.value)
    )
}

/// One block as a `RustSyntax.Block` term.
#[must_use]
pub fn block(written: &Block) -> String {
    format!(
        "(.mk {} {})",
        list(written.lets.iter().map(binding)),
        expr(&written.tail)
    )
}

fn capture(read: CaptureRead) -> &'static str {
    match read {
        CaptureRead::Copy => ".copy",
        CaptureRead::Clone => ".clone",
        CaptureRead::Unbox => ".unbox",
        CaptureRead::Unit => ".unit",
    }
}

fn dispatch(arm: &Dispatch) -> String {
    format!(
        "{{ function := {}, captures := {}, functionFallible := {} }}",
        arm.function,
        list(arm.captures.iter().map(|read| capture(*read).to_owned())),
        arm.function_fallible
    )
}

fn item_def(definition: &ItemDef) -> String {
    match definition {
        ItemDef::Enum {
            name,
            variants,
            at: _,
        } => format!(
            "(.enum {} {})",
            ty(name),
            list(variants.iter().map(|(number, fields)| format!(
                "{{ number := {number}, fields := {} }}",
                list(fields.iter().map(ty))
            )))
        ),
        ItemDef::Apply {
            fn_type,
            parameters,
            result,
            fallible,
            arms,
            at: _,
        } => format!(
            "(.apply {fn_type} {} {} {fallible} {})",
            list(parameters.iter().map(ty)),
            ty(result),
            list(arms.iter().map(dispatch))
        ),
        ItemDef::Function {
            name,
            parameters,
            result,
            body,
            at: _,
        } => format!(
            "(.function {} {} {} {})",
            ident(name),
            list(parameters.iter().map(|(pattern, declared)| format!(
                "{{ pattern := {}, type := {} }}",
                pat(pattern),
                ty(declared)
            ))),
            ty(result),
            block(body)
        ),
    }
}

/// A crate as a `RustSyntax.Crate` term, one item per line.
#[must_use]
pub fn crate_term(rendered: &Crate) -> String {
    let profile = match rendered.profile {
        Profile::Core => ".core",
        Profile::Std => ".std",
    };
    let items: Vec<String> = rendered.items.iter().map(item_def).collect();
    format!(
        "({{ profile := {profile}, items := [\n    {}] }} : {RUST_SYNTAX}.Crate)",
        items.join(",\n    ")
    )
}
