//! The typing sites of a valid program, as the lowering reads them: the type
//! of each match scrutinee and of each applied target (`trees`), and the
//! types the lowering of terms reads besides (`uses`): those of the operands
//! of a primitive and of the record a field is read from. The tree has the
//! shape of the program's own chains, so a consumer that walks the program
//! walks the tree beside it (`LexLeanBoot.RustLower`).

use super::check::Checker;
use super::{Expr, Function, Program, Ty};

/// One node of the tree. A node without children is a leaf; `own` is the
/// type a match or an application reads at that node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Site {
    Leaf,
    Node { own: Option<Ty>, kids: Vec<Site> },
}

/// Which sites a tree states.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    /// The scrutinee of each match and the target of each application.
    Sites,
    /// The operands of each primitive and the value of each field.
    Uses,
}

impl Site {
    fn node(own: Option<Ty>, kids: Vec<Self>) -> Self {
        Self::Node { own, kids }
    }

    /// This tree with `ty` as the type its node reads.
    fn read(self, ty: Ty) -> Self {
        match self {
            Self::Leaf => Self::Node {
                own: Some(ty),
                kids: Vec::new(),
            },
            Self::Node { kids, .. } => Self::Node {
                own: Some(ty),
                kids,
            },
        }
    }

    /// A chain of nodes the way the program states a chain of terms: a node
    /// holding the head and the rest, and a leaf at its end.
    fn chain(items: Vec<Self>) -> Self {
        items
            .into_iter()
            .rev()
            .fold(Self::Leaf, |rest, head| Self::node(None, vec![head, rest]))
    }
}

/// The operands of a term; `typed` makes each of them read its own type.
fn operands(
    kind: Kind,
    checker: &Checker<'_>,
    exprs: &[Expr],
    typed: bool,
    scope: &mut Vec<(u64, Ty)>,
) -> Result<Site, String> {
    let mut items = Vec::new();
    for expr in exprs {
        let tree = site(kind, checker, expr, scope)?;
        items.push(if typed {
            tree.read(checker.expr(expr, scope)?)
        } else {
            tree
        });
    }
    Ok(Site::chain(items))
}

fn site(
    kind: Kind,
    checker: &Checker<'_>,
    expr: &Expr,
    scope: &mut Vec<(u64, Ty)>,
) -> Result<Site, String> {
    Ok(match expr {
        Expr::Value { .. } | Expr::Var { .. } => Site::Leaf,
        Expr::Let {
            name,
            ty,
            bound,
            body,
        } => {
            let bound = site(kind, checker, bound, scope)?;
            scope.push((*name, ty.clone()));
            let body = site(kind, checker, body, scope);
            scope.pop();
            Site::node(None, vec![bound, body?])
        }
        Expr::Cond {
            condition,
            then_branch,
            else_branch,
        } => Site::node(
            None,
            vec![
                site(kind, checker, condition, scope)?,
                site(kind, checker, then_branch, scope)?,
                site(kind, checker, else_branch, scope)?,
            ],
        ),
        Expr::Match {
            scrutinee, arms, ..
        } => {
            let scrutinee_ty = checker.expr(scrutinee, scope)?;
            let first = site(kind, checker, scrutinee, scope)?;
            let mut items = Vec::new();
            for arm in arms {
                let fields = checker.shape_fields(arm.shape, &scrutinee_ty)?;
                let depth = scope.len();
                scope.extend(arm.binders.iter().copied().zip(fields));
                let body = site(kind, checker, &arm.body, scope);
                scope.truncate(depth);
                items.push(Site::node(None, vec![body?]));
            }
            Site::node(
                (kind == Kind::Sites).then_some(scrutinee_ty),
                vec![first, Site::chain(items)],
            )
        }
        Expr::Build { operands: list, .. } | Expr::Call { operands: list, .. } => {
            Site::node(None, vec![operands(kind, checker, list, false, scope)?])
        }
        Expr::Prim { operands: list, .. } => Site::node(
            None,
            vec![operands(kind, checker, list, kind == Kind::Uses, scope)?],
        ),
        Expr::Closure { captures, .. } => {
            Site::node(None, vec![operands(kind, checker, captures, false, scope)?])
        }
        Expr::Apply {
            target,
            operands: list,
        } => {
            let target_ty = checker.expr(target, scope)?;
            Site::node(
                (kind == Kind::Sites).then_some(target_ty),
                vec![
                    site(kind, checker, target, scope)?,
                    operands(kind, checker, list, false, scope)?,
                ],
            )
        }
        Expr::First { value } | Expr::Second { value } => {
            Site::node(None, vec![site(kind, checker, value, scope)?])
        }
        Expr::Field { value, .. } => {
            let tree = site(kind, checker, value, scope)?;
            Site::node(
                None,
                vec![if kind == Kind::Uses {
                    tree.read(checker.expr(value, scope)?)
                } else {
                    tree
                }],
            )
        }
    })
}

fn scope_of(function: &Function) -> Vec<(u64, Ty)> {
    function
        .parameters
        .iter()
        .copied()
        .zip(function.types.iter().cloned())
        .collect()
}

fn bodies(program: &Program, kind: Kind) -> Result<Site, String> {
    super::check::check(program)?;
    let checker = Checker { program };
    let mut items = Vec::new();
    for function in &program.functions {
        items.push(site(
            kind,
            &checker,
            &function.body,
            &mut scope_of(function),
        )?);
    }
    Ok(Site::chain(items))
}

/// The tree of every function body of a valid program, chained as the bodies
/// are, with the type of each match scrutinee and of each applied target.
///
/// # Errors
///
/// Returns the reason the program is invalid.
pub fn trees(program: &Program) -> Result<Site, String> {
    bodies(program, Kind::Sites)
}

/// The same tree with the types of the operands of each primitive and of the
/// value each field is read from.
///
/// # Errors
///
/// Returns the reason the program is invalid.
pub fn uses(program: &Program) -> Result<Site, String> {
    bodies(program, Kind::Uses)
}
