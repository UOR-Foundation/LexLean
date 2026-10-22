//! `BS-07`, third slice, first pass: last-use moves of SPEC.md §17.16
//! (`calculus::rust::moves::apply`) as language-1.2 source
//! (`LexLeanBoot.RustMoves`), held to the oracle to the byte.
//!
//! The root takes the items of a crate as the `Def` chain of `RustAst` and
//! returns them with every function body rewritten. `apply` is public, so
//! this pass is held to it directly on any block; the corpora are crates, the
//! hand-built crates of every rule of the pass and the crates of `BS-05`.

use lexlean::calculus::rust::ast::{
    Block, Callee, Crate, Expr, Ident, ItemDef, Let, Lit, Origin, Pat,
};
use lexlean::calculus::rust::moves;
use lexlean::calculus::rust::Profile;

use super::rust_print::enc_items;
use super::rust_validate::{
    branch_crates, committed, loop_crates, machine_crates, mutant, pool_of, rendered, small,
};
use super::{Run, Seeded};

/// The crates a mutation draws on have at most this many expressions.
const BASE_NODES: usize = 80;

fn at() -> Origin {
    Origin::of("hand")
}

fn local(n: u64) -> Ident {
    Ident::Local(n)
}

fn cloned(n: u64) -> Expr {
    Expr::Clone(local(n), at())
}

fn unit() -> Expr {
    Expr::Lit(Lit::Unit, at())
}

fn block(lets: Vec<Let>, tail: Expr) -> Block {
    Block { lets, tail }
}

fn bind(n: u64, value: Expr) -> Let {
    Let {
        pat: Pat::Bind(local(n)),
        ty: None,
        value,
        at: at(),
    }
}

/// `let (h, _) = ...` or `let (_, h) = ...` of a pair, the projection the pass
/// moves out of a pair.
fn project(part: usize, n: u64, value: Expr) -> Let {
    let mut parts = vec![Pat::Wild, Pat::Wild];
    parts[part] = Pat::Bind(local(n));
    Let {
        pat: Pat::Tuple(parts),
        ty: None,
        value,
        at: at(),
    }
}

fn pair(left: Expr, right: Expr) -> Expr {
    Expr::Pair(Box::new(left), Box::new(right), at())
}

fn call(args: Vec<Expr>) -> Expr {
    Expr::Call {
        callee: Callee::Function(0),
        args,
        propagate: false,
        at: at(),
    }
}

fn function(body: Block) -> ItemDef {
    ItemDef::Function {
        name: Ident::Function(0),
        parameters: Vec::new(),
        result: lexlean::calculus::rust::ast::Type::Unit,
        body,
        at: at(),
    }
}

fn krate(body: Block) -> Crate {
    Crate {
        profile: Profile::Std,
        items: vec![function(body)],
    }
}

fn arm(pattern: Pat, body: Block) -> (Pat, Block) {
    (pattern, body)
}

/// The crates of the rules of the pass: a read that is the last, one that is
/// not, the projection of a pair component and of both, alternatives, a match
/// with and without arms, a loop, an application, and the blocks in between.
fn rule_crates() -> Vec<Crate> {
    let conditional = |condition: Expr, then_branch: Block, else_branch: Block| Expr::If {
        condition: Box::new(condition),
        then_branch: Box::new(then_branch),
        else_branch: Box::new(else_branch),
        at: at(),
    };
    let matched = |scrutinee: Expr, arms: Vec<(Pat, Block)>| Expr::Match {
        scrutinee: Box::new(scrutinee),
        arms,
        at: at(),
    };
    vec![
        // The only read.
        krate(block(Vec::new(), cloned(0))),
        // Read, then read again: only the second can move.
        krate(block(vec![bind(1, cloned(0))], pair(cloned(0), cloned(1)))),
        // The same read twice in one call.
        krate(block(Vec::new(), call(vec![cloned(0), cloned(0)]))),
        // A bound value read once, and one read once and then bound again.
        krate(block(
            vec![bind(1, cloned(0)), bind(0, cloned(1))],
            cloned(0),
        )),
        // The projection of the first component, alone.
        krate(block(vec![project(0, 1, cloned(0))], cloned(1))),
        // The projection of the second component, with the pair read whole
        // afterwards.
        krate(block(
            vec![project(1, 1, cloned(0))],
            pair(cloned(1), cloned(0)),
        )),
        // Both components projected, each once.
        krate(block(
            vec![project(0, 1, cloned(0)), project(1, 2, cloned(0))],
            pair(cloned(1), cloned(2)),
        )),
        // A component projected twice.
        krate(block(
            vec![project(0, 1, cloned(0)), project(0, 2, cloned(0))],
            pair(cloned(1), cloned(2)),
        )),
        // A component projected while another is read twice after it: the
        // pair cannot lose a component that the later reads of the other
        // would need whole.
        krate(block(
            vec![
                project(0, 1, cloned(0)),
                project(1, 2, cloned(0)),
                project(1, 3, cloned(0)),
            ],
            pair(pair(cloned(1), cloned(2)), cloned(3)),
        )),
        // A component read once on each of two alternatives is read once, so
        // the other component may be projected before them.
        krate(block(
            vec![project(1, 1, cloned(0))],
            conditional(
                cloned(5),
                block(vec![project(0, 2, cloned(0))], cloned(2)),
                block(vec![project(0, 3, cloned(0))], cloned(3)),
            ),
        )),
        // A projection of a pattern that binds both, which is not one.
        krate(block(
            vec![Let {
                pat: Pat::Tuple(vec![Pat::Bind(local(1)), Pat::Bind(local(2))]),
                ty: None,
                value: cloned(0),
                at: at(),
            }],
            pair(cloned(1), cloned(2)),
        )),
        // A projection followed by a pattern that binds both components: the
        // second reads the pair whole, so the first cannot move it.
        krate(block(
            vec![
                project(1, 1, cloned(0)),
                Let {
                    pat: Pat::Tuple(vec![Pat::Bind(local(2)), Pat::Bind(local(3))]),
                    ty: None,
                    value: cloned(0),
                    at: at(),
                },
            ],
            pair(pair(cloned(1), cloned(2)), cloned(3)),
        )),
        // Alternatives: a read in one branch only, in both, and after.
        krate(block(
            Vec::new(),
            conditional(
                cloned(3),
                block(Vec::new(), cloned(0)),
                block(Vec::new(), unit()),
            ),
        )),
        krate(block(
            Vec::new(),
            conditional(
                cloned(3),
                block(Vec::new(), cloned(0)),
                block(Vec::new(), cloned(0)),
            ),
        )),
        krate(block(
            vec![bind(
                1,
                conditional(
                    cloned(3),
                    block(Vec::new(), cloned(0)),
                    block(Vec::new(), unit()),
                ),
            )],
            pair(cloned(1), cloned(0)),
        )),
        // The condition read again in a branch.
        krate(block(
            Vec::new(),
            conditional(
                cloned(0),
                block(Vec::new(), cloned(0)),
                block(Vec::new(), unit()),
            ),
        )),
        // A match: the scrutinee, an arm that binds, an arm that reads
        // outside, and a pattern that rebinds a name read after.
        krate(block(
            Vec::new(),
            matched(
                cloned(0),
                vec![
                    arm(
                        Pat::Some(Box::new(Pat::Bind(local(1)))),
                        block(Vec::new(), cloned(1)),
                    ),
                    arm(Pat::None, block(Vec::new(), cloned(2))),
                ],
            ),
        )),
        krate(block(
            vec![bind(
                3,
                matched(
                    cloned(0),
                    vec![
                        arm(Pat::Bind(local(0)), block(Vec::new(), cloned(0))),
                        arm(Pat::Wild, block(Vec::new(), cloned(4))),
                    ],
                ),
            )],
            pair(cloned(3), cloned(0)),
        )),
        // A match without arms.
        krate(block(Vec::new(), matched(cloned(0), Vec::new()))),
        // An application: the holder is read before the operands.
        krate(block(
            Vec::new(),
            Expr::Apply {
                holder: Ident::Callee(0),
                args: vec![cloned(1), cloned(0)],
                propagate: false,
                at: at(),
            },
        )),
        krate(block(
            vec![bind(1, cloned(2))],
            Expr::Apply {
                holder: local(2),
                args: vec![cloned(1)],
                propagate: true,
                at: at(),
            },
        )),
        // The closure applied is also an operand: it is borrowed for the
        // whole application, so the operand cannot move it.
        krate(block(
            Vec::new(),
            Expr::Apply {
                holder: local(0),
                args: vec![cloned(0)],
                propagate: false,
                at: at(),
            },
        )),
        // Loops: a read of the state, of another binding, and of both.
        krate(block(
            Vec::new(),
            Expr::Loop {
                state: local(0),
                body: Box::new(block(vec![bind(1, cloned(0))], cloned(2))),
            },
        )),
        krate(block(
            vec![bind(5, cloned(2))],
            pair(
                Expr::Loop {
                    state: local(0),
                    body: Box::new(block(Vec::new(), pair(cloned(0), cloned(2)))),
                },
                cloned(2),
            ),
        )),
        // A read of the state before the loop that rebinds it: the loop reads
        // it whole, so the earlier read cannot move it.
        krate(block(
            vec![bind(1, cloned(0))],
            Expr::Loop {
                state: local(0),
                body: Box::new(block(Vec::new(), cloned(1))),
            },
        )),
        // Blocks as expressions, and wrappers.
        krate(block(
            Vec::new(),
            Expr::Block(Box::new(block(vec![bind(1, cloned(0))], cloned(1)))),
        )),
        krate(block(
            Vec::new(),
            Expr::Not(
                Box::new(Expr::Box(Box::new(pair(cloned(0), cloned(0))), at())),
                at(),
            ),
        )),
        // Reads that are not clones.
        krate(block(
            vec![bind(1, Expr::Copy(local(0), at()))],
            pair(cloned(0), Expr::Deref(local(1), at())),
        )),
    ]
}

/// The oracle's move of every function body of a crate, as the root takes and
/// returns the items.
pub(super) fn run(krate: &Crate) -> Run {
    let input = enc_items(&krate.items);
    let mut after = krate.clone();
    for item in &mut after.items {
        if let ItemDef::Function { body, .. } = item {
            moves::apply(body);
        }
    }
    (
        vec![input],
        serde_json::to_value(enc_items(&after.items)).expect("a value is JSON"),
    )
}

/// The crates the pass is run on: the rule crates, the hand-built crates of
/// `BS-05`, and the crates the renderer lowers from the committed programs.
pub(super) fn base() -> Vec<Crate> {
    let mut out = rule_crates();
    out.extend(branch_crates());
    out.extend(machine_crates());
    out.extend(loop_crates());
    let (sources, ..) = committed();
    out.extend(rendered(&sources).into_iter().map(|one| one.krate));
    out
}

/// `count` seeded mutants of the crates the renderer lowers from the
/// committed programs.
pub(super) fn mutants(count: usize, seed: u64) -> Vec<Crate> {
    let (sources, ..) = committed();
    let lowered = rendered(&sources);
    // The mutants of the compiler's own crates are too large to hold by the
    // thousand: the bases are the crates of at most `BASE_NODES` expressions,
    // as `BS-05` draws its mutants.
    let bases = small(&lowered, BASE_NODES);
    let mut seeded = Seeded(seed);
    bases
        .iter()
        .cycle()
        .take(count)
        .map(|one| mutant(&one.krate, &mut seeded, &pool_of(&one.krate)))
        .collect()
}
