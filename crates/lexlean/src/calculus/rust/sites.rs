//! The typing sites of a valid program, as the lowering's failure analysis
//! reads them: the type of each match scrutinee and of each applied target.
//! The tree has the shape of the program's own chains, so a consumer that
//! walks the program walks the tree beside it (`LexLeanBoot.RustLower`).

use super::super::check::Checker;
use super::super::{Expr, Function, Program, Ty};

/// One node of the tree. A node without children is a leaf; `own` is the
/// type a match or an application reads at that node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Site {
    Leaf,
    Node { own: Option<Ty>, kids: Vec<Site> },
}

impl Site {
    fn node(own: Option<Ty>, kids: Vec<Self>) -> Self {
        Self::Node { own, kids }
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

fn operands(
    checker: &Checker<'_>,
    exprs: &[Expr],
    scope: &mut Vec<(u64, Ty)>,
) -> Result<Site, String> {
    let mut items = Vec::new();
    for expr in exprs {
        items.push(site(checker, expr, scope)?);
    }
    Ok(Site::chain(items))
}

fn site(checker: &Checker<'_>, expr: &Expr, scope: &mut Vec<(u64, Ty)>) -> Result<Site, String> {
    Ok(match expr {
        Expr::Value { .. } | Expr::Var { .. } => Site::Leaf,
        Expr::Let {
            name,
            ty,
            bound,
            body,
        } => {
            let bound = site(checker, bound, scope)?;
            scope.push((*name, ty.clone()));
            let body = site(checker, body, scope);
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
                site(checker, condition, scope)?,
                site(checker, then_branch, scope)?,
                site(checker, else_branch, scope)?,
            ],
        ),
        Expr::Match {
            scrutinee, arms, ..
        } => {
            let scrutinee_ty = checker.expr(scrutinee, scope)?;
            let first = site(checker, scrutinee, scope)?;
            let mut items = Vec::new();
            for arm in arms {
                let fields = checker.shape_fields(arm.shape, &scrutinee_ty)?;
                let depth = scope.len();
                scope.extend(arm.binders.iter().copied().zip(fields));
                let body = site(checker, &arm.body, scope);
                scope.truncate(depth);
                items.push(Site::node(None, vec![body?]));
            }
            Site::node(Some(scrutinee_ty), vec![first, Site::chain(items)])
        }
        Expr::Build { operands: list, .. }
        | Expr::Call { operands: list, .. }
        | Expr::Prim { operands: list, .. } => {
            Site::node(None, vec![operands(checker, list, scope)?])
        }
        Expr::Closure { captures, .. } => {
            Site::node(None, vec![operands(checker, captures, scope)?])
        }
        Expr::Apply {
            target,
            operands: list,
        } => {
            let target_ty = checker.expr(target, scope)?;
            Site::node(
                Some(target_ty),
                vec![
                    site(checker, target, scope)?,
                    operands(checker, list, scope)?,
                ],
            )
        }
        Expr::First { value } | Expr::Second { value } | Expr::Field { value, .. } => {
            Site::node(None, vec![site(checker, value, scope)?])
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

/// The tree of every function body of a valid program, chained as the bodies
/// are.
pub(super) fn trees(program: &Program) -> Result<Site, String> {
    super::super::check::check(program)?;
    let checker = Checker { program };
    let mut items = Vec::new();
    for function in &program.functions {
        items.push(site(&checker, &function.body, &mut scope_of(function))?);
    }
    Ok(Site::chain(items))
}
