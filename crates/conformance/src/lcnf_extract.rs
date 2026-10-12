//! The second half of the extraction adapter's authoring form (see
//! `lcnf_adapter`): the facts of a constant, the signature probe a command
//! runs, the listing and the check of the constants the adapter itself uses,
//! and the extraction worklist.
//!
//! `run` is the old adapter's `while` loop with its mutable state, as
//! recursion over explicit state: a tooling module has no `do` and no mutable
//! variable (SPEC.md §17.20). The two queues of the loop become two phases,
//! which compute the same sets: every name the code of a translated
//! definition reaches in the project is visited and translated first, and the
//! kernel values of everything visited then close the set of recorded
//! constants, which the original reached by always taking from the code queue
//! before the kernel queue.

use serde_json::Value as Json;

use crate::lcnf_adapter::{
    and, arr_list, bind, bool_of, cat, cons, eq, field_of, get_env, kind_of, m_command, m_core,
    m_term, map_list, name_of, nat_text, nil, not, obj, or, own, pair, pure_in, s, simple_name,
    str_of, t_array, t_bool, t_command, t_core, t_env, t_expr, t_list, t_name, t_pair, t_prod,
    t_pure, t_set, t_string, t_unit, throw, unit_value,
};
use crate::tl::{
    array, branch, call, construct, function, if_then, lambda, let_in, list, matching, nat,
    otherwise, ty, ty_app, var, Flags, PARTIAL, PUBLIC,
};

fn t_info() -> Json {
    ty("Lean.ConstantInfo")
}

fn t_decls() -> Json {
    ty_app("Std.HashMap", vec![t_name(), t_string()])
}

/// The state the code phase returns: visited names, declaration records, and
/// the constants of other modules the translated code refers to.
fn t_visit() -> Json {
    t_prod(t_set(), t_prod(t_decls(), t_set()))
}

fn t_classed() -> Json {
    t_prod(t_name(), t_string())
}

fn t_registered_types() -> Json {
    t_array(t_prod(t_name(), t_array(t_string())))
}

fn empty_set() -> Json {
    call("Std.HashSet.emptyWithCapacity", vec![t_name()], vec![])
}

fn empty_map() -> Json {
    call(
        "Std.HashMap.emptyWithCapacity",
        vec![t_name(), t_string()],
        vec![],
    )
}

fn set_insert(set: Json, element: Json) -> Json {
    call(&own("insert"), vec![], vec![set, element])
}

fn set_contains(set: Json, element: Json) -> Json {
    call("Std.HashSet.contains", vec![t_name()], vec![set, element])
}

fn find(env: Json, n: Json) -> Json {
    call("Lean.Environment.find?", vec![], vec![env, n])
}

fn name_text(n: Json) -> Json {
    call("Lean.Name.toString", vec![], vec![n])
}

fn list_append(element: Json, left: Json, right: Json) -> Json {
    call("List.append", vec![element], vec![left, right])
}

fn is_some(element: Json, value: Json) -> Json {
    call("Option.isSome", vec![element], vec![value])
}

fn string_eq(left: Json, right: Json) -> Json {
    eq(t_string(), left, right)
}

fn name_eq(left: Json, right: Json) -> Json {
    eq(t_name(), left, right)
}

fn any_of(mut terms: Vec<Json>) -> Json {
    let mut result = terms.pop().expect("a term");
    while let Some(term) = terms.pop() {
        result = or(term, result);
    }
    result
}

fn info_branch(constructor: &str, binders: &[&str], body: Json) -> Json {
    branch(&format!("Lean.ConstantInfo.{constructor}"), binders, body)
}

fn unresolved(n: &str) -> Json {
    cat(vec![
        s("lexlean-extract: unresolved constant `"),
        name_text(var(n)),
        s("`"),
    ])
}

fn pairs(items: Vec<(&str, Json)>) -> Json {
    list(
        t_pair(),
        items
            .into_iter()
            .map(|(key, value)| pair(key, value))
            .collect(),
    )
}

/// Constants' facts: kind, module, and whether Lean compiles them.
pub fn info_functions() -> Vec<Json> {
    let safety = matching(
        field_of(var("value"), "safety"),
        vec![
            branch("Lean.DefinitionSafety.safe", &[], s("definition")),
            branch("Lean.DefinitionSafety.unsafe", &[], s("unsafe-definition")),
            branch(
                "Lean.DefinitionSafety.partial",
                &[],
                s("partial-definition"),
            ),
        ],
    );
    let kind_body = matching(
        var("info"),
        vec![
            info_branch("defnInfo", &["value"], safety),
            info_branch("opaqueInfo", &["opaqueValue"], s("opaque")),
            info_branch("thmInfo", &["theoremValue"], s("theorem")),
            info_branch("axiomInfo", &["axiomValue"], s("axiom")),
            info_branch("inductInfo", &["inductiveValue"], s("inductive")),
            info_branch("ctorInfo", &["constructorValue"], s("constructor")),
            info_branch("recInfo", &["recursorValue"], s("recursor")),
            info_branch("quotInfo", &["quotientValue"], s("quotient")),
        ],
    );
    let kind_row = |constructor: &str, text: &str| {
        branch(&format!("Lean.ConstantKind.{constructor}"), &[], s(text))
    };
    let original_body = matching(
        call(
            "Lean.getOriginalConstKind?",
            vec![],
            vec![var("env"), var("n")],
        ),
        vec![
            branch(
                "Option.some",
                &["original"],
                matching(
                    var("original"),
                    vec![
                        kind_row("defn", "definition"),
                        kind_row("thm", "theorem"),
                        kind_row("axiom", "axiom"),
                        kind_row("opaque", "opaque"),
                        kind_row("quot", "quotient"),
                        kind_row("induct", "inductive"),
                        kind_row("ctor", "constructor"),
                        kind_row("recursor", "recursor"),
                    ],
                ),
            ),
            branch("Option.none", &[], s("unknown")),
        ],
    );
    let anonymous = || construct("Lean.Name.anonymous", vec![], vec![]);
    let module_body = matching(
        call(
            "Lean.Environment.getModuleIdxFor?",
            vec![],
            vec![var("env"), var("n")],
        ),
        vec![
            branch(
                "Option.some",
                &["index"],
                call(
                    "Array.getD",
                    vec![t_name()],
                    vec![
                        call(
                            "Lean.EnvironmentHeader.moduleNames",
                            vec![],
                            vec![call("Lean.Environment.header", vec![], vec![var("env")])],
                        ),
                        var("index"),
                        anonymous(),
                    ],
                ),
            ),
            branch("Option.none", &[], anonymous()),
        ],
    );
    let facts_row = pairs(vec![
        ("name", name_of(var("n"))),
        (
            "kind",
            str_of(call(&own("kind"), vec![], vec![var("info")])),
        ),
        (
            "module",
            name_of(call(&own("moduleOf"), vec![], vec![var("env"), var("n")])),
        ),
        (
            "computable",
            bool_of(not(call(
                "Lean.isNoncomputable",
                vec![],
                vec![var("env"), var("n")],
            ))),
        ),
        ("generates_code", bool_of(var("generates"))),
        (
            "original_kind",
            str_of(call(
                &own("originalKind"),
                vec![],
                vec![var("env"), var("n")],
            )),
        ),
        (
            "internal",
            bool_of(call("Lean.Name.isInternal", vec![], vec![var("n")])),
        ),
    ]);
    let facts_body = bind(
        &m_core(),
        t_bool(),
        t_list(t_pair()),
        call(
            "Lean.Compiler.LCNF.shouldGenerateCode",
            vec![],
            vec![var("n")],
        ),
        "generates",
        pure_in(&m_core(), t_list(t_pair()), facts_row),
    );
    vec![
        function(
            &own("kind"),
            &[],
            &[("info", t_info())],
            t_string(),
            kind_body,
            Flags::default(),
            None,
        ),
        function(
            &own("originalKind"),
            &[],
            &[("env", t_env()), ("n", t_name())],
            t_string(),
            original_body,
            Flags::default(),
            None,
        ),
        function(
            &own("moduleOf"),
            &[],
            &[("env", t_env()), ("n", t_name())],
            t_name(),
            module_body,
            Flags::default(),
            None,
        ),
        function(
            &own("inProject"),
            &[],
            &[
                ("env", t_env()),
                ("modules", t_array(t_name())),
                ("n", t_name()),
            ],
            t_bool(),
            call(
                "Array.contains",
                vec![t_name()],
                vec![
                    var("modules"),
                    call(&own("moduleOf"), vec![], vec![var("env"), var("n")]),
                ],
            ),
            Flags::default(),
            None,
        ),
        function(
            &own("facts"),
            &[],
            &[("env", t_env()), ("n", t_name()), ("info", t_info())],
            t_core(t_list(t_pair())),
            facts_body,
            Flags::default(),
            None,
        ),
    ]
}

/// The signature probe: a registered signature, elaborated by Lean, must
/// equal the type the constant has (binder kinds and defaults included).
pub fn signature_functions() -> Vec<Json> {
    let expr = |constructor: &str, binders: &[&str], body: Json| {
        branch(&format!("Lean.Expr.{constructor}"), binders, body)
    };
    let same = |left: Json, right: Json| call(&own("sameSignature"), vec![], vec![left, right]);
    let info_eq = |left: &str, right: &str| eq(ty("Lean.BinderInfo"), var(left), var(right));
    let falsity = || construct("Bool.false", vec![], vec![]);
    let two_sorted = |ctor: &str| {
        expr(
            ctor,
            &["name1", "domain1", "body1", "info1"],
            matching(
                var("b"),
                vec![
                    expr(
                        ctor,
                        &["name2", "domain2", "body2", "info2"],
                        and(
                            info_eq("info1", "info2"),
                            and(
                                same(var("domain1"), var("domain2")),
                                same(var("body1"), var("body2")),
                            ),
                        ),
                    ),
                    otherwise(falsity()),
                ],
            ),
        )
    };
    let single = |ctor: &str, left: &[&str], right: &[&str], body: Json| {
        expr(
            ctor,
            left,
            matching(
                var("b"),
                vec![expr(ctor, right, body), otherwise(falsity())],
            ),
        )
    };
    let structure_body = matching(
        var("a"),
        vec![
            two_sorted("forallE"),
            two_sorted("lam"),
            single(
                "app",
                &["function1", "argument1"],
                &["function2", "argument2"],
                and(
                    same(var("function1"), var("function2")),
                    same(var("argument1"), var("argument2")),
                ),
            ),
            single(
                "const",
                &["constant1", "levels1"],
                &["constant2", "levels2"],
                and(
                    name_eq(var("constant1"), var("constant2")),
                    eq(t_list(ty("Lean.Level")), var("levels1"), var("levels2")),
                ),
            ),
            single(
                "sort",
                &["level1"],
                &["level2"],
                eq(ty("Lean.Level"), var("level1"), var("level2")),
            ),
            single(
                "bvar",
                &["position1"],
                &["position2"],
                eq(ty("Nat"), var("position1"), var("position2")),
            ),
            single(
                "lit",
                &["literal1"],
                &["literal2"],
                eq(ty("Lean.Literal"), var("literal1"), var("literal2")),
            ),
            single(
                "fvar",
                &["free1"],
                &["free2"],
                eq(ty("Lean.FVarId"), var("free1"), var("free2")),
            ),
            single(
                "mvar",
                &["meta1"],
                &["meta2"],
                eq(ty("Lean.MVarId"), var("meta1"), var("meta2")),
            ),
            single(
                "letE",
                &["local1", "type1", "value1", "scope1", "flag1"],
                &["local2", "type2", "value2", "scope2", "flag2"],
                and(
                    same(var("type1"), var("type2")),
                    and(
                        same(var("value1"), var("value2")),
                        same(var("scope1"), var("scope2")),
                    ),
                ),
            ),
            single(
                "proj",
                &["structure1", "field1", "target1"],
                &["structure2", "field2", "target2"],
                and(
                    name_eq(var("structure1"), var("structure2")),
                    and(
                        eq(ty("Nat"), var("field1"), var("field2")),
                        same(var("target1"), var("target2")),
                    ),
                ),
            ),
            otherwise(falsity()),
        ],
    );
    let signature_body = matching(
        var("a"),
        vec![
            expr("mdata", &["data1", "inner1"], same(var("inner1"), var("b"))),
            otherwise(matching(
                var("b"),
                vec![
                    expr("mdata", &["data2", "inner2"], same(var("a"), var("inner2"))),
                    otherwise(call(
                        &own("sameStructure"),
                        vec![],
                        vec![var("a"), var("b")],
                    )),
                ],
            )),
        ],
    );
    let group = Some("Signature");
    let message = cat_message(vec![
        MessagePart::Text("lexlean-extract-drift: the signature of `"),
        MessagePart::Name(var("declName")),
        MessagePart::Text("` is "),
        MessagePart::Expr(call("Lean.ConstantInfo.type", vec![], vec![var("info")])),
    ]);
    let probe = lambda(
        &[("stx", ty("Lean.Syntax"))],
        call(
            "Lean.Elab.Command.liftTermElabM",
            vec![t_unit()],
            vec![let_in(
                "declName",
                t_name(),
                call(
                    "Lean.Syntax.getId",
                    vec![],
                    vec![call("Lean.Syntax.getArg", vec![], vec![var("stx"), nat(1)])],
                ),
                bind(
                    &m_term(),
                    t_info(),
                    t_unit(),
                    call("Lean.getConstInfo", vec![m_term()], vec![var("declName")]),
                    "info",
                    bind(
                        &m_term(),
                        t_expr(),
                        t_unit(),
                        call(
                            "Lean.Elab.Term.elabType",
                            vec![],
                            vec![call("Lean.Syntax.getArg", vec![], vec![var("stx"), nat(3)])],
                        ),
                        "elaborated",
                        bind(
                            &m_term(),
                            t_expr(),
                            t_unit(),
                            call(
                                "Lean.instantiateMVars",
                                vec![m_term()],
                                vec![var("elaborated")],
                            ),
                            "registered",
                            bind(
                                &m_term(),
                                t_expr(),
                                t_unit(),
                                call(
                                    "Lean.instantiateMVars",
                                    vec![m_term()],
                                    vec![call("Lean.ConstantInfo.type", vec![], vec![var("info")])],
                                ),
                                "actual",
                                if_then(
                                    same(var("actual"), var("registered")),
                                    pure_in(&m_term(), t_unit(), unit_value()),
                                    call(
                                        "Lean.throwError",
                                        vec![m_term(), t_unit()],
                                        vec![message],
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            )],
        ),
    );
    vec![
        function(
            &own("sameSignature"),
            &[],
            &[("a", t_expr()), ("b", t_expr())],
            t_bool(),
            signature_body,
            PARTIAL,
            group,
        ),
        function(
            &own("sameStructure"),
            &[],
            &[("a", t_expr()), ("b", t_expr())],
            t_bool(),
            structure_body,
            PARTIAL,
            group,
        ),
        function(
            &own("elabSignature"),
            &[],
            &[],
            ty("Lean.Elab.Command.CommandElab"),
            probe,
            PUBLIC,
            None,
        ),
    ]
}

enum MessagePart {
    Text(&'static str),
    Name(Json),
    Expr(Json),
}

/// A message built of text, names, and expressions: the same `MessageData`
/// the old adapter's interpolated `throwError` built.
fn cat_message(parts: Vec<MessagePart>) -> Json {
    let data = |part: MessagePart| match part {
        MessagePart::Text(text) => call("Lean.stringToMessageData", vec![], vec![s(text)]),
        MessagePart::Name(value) => call("Lean.MessageData.ofName", vec![], vec![value]),
        MessagePart::Expr(value) => call("Lean.MessageData.ofExpr", vec![], vec![value]),
    };
    let mut parts: Vec<Json> = parts.into_iter().map(data).collect();
    let mut result = parts.pop().expect("a part");
    while let Some(part) = parts.pop() {
        result = call("Lean.MessageData.compose", vec![], vec![part, result]);
    }
    result
}

/// The constants the adapter's own definitions use, classified by what the
/// registry must say of each (call, type, or plumbing).
pub fn listing_functions() -> Vec<Json> {
    let command = m_command();
    let classed_list = t_list(t_classed());
    let lean_owned = || {
        and(
            call(
                "Lean.Name.isPrefixOf",
                vec![],
                vec![simple_name("Lean"), var("c")],
            ),
            not(var("structural")),
        )
    };
    let call_or_plumbing = |condition: Json| if_then(condition, s("call"), s("plumbing"));
    let classify_body = let_in(
        "structural",
        t_bool(),
        any_of(vec![
            call(
                "Lean.Environment.isProjectionFn",
                vec![],
                vec![var("env"), var("c")],
            ),
            call("Lean.isCasesOnRecursor", vec![], vec![var("env"), var("c")]),
            call("Lean.isAuxRecursor", vec![], vec![var("env"), var("c")]),
            call("Lean.isNoConfusion", vec![], vec![var("env"), var("c")]),
            call("Lean.Name.isInternal", vec![], vec![var("c")]),
            call(
                "Lean.Meta.isInstanceCore",
                vec![],
                vec![var("env"), var("c")],
            ),
            call(
                "Lean.Meta.isMatcherCore",
                vec![],
                vec![var("env"), var("c")],
            ),
            string_eq(
                call("Lean.Name.getString!", vec![], vec![var("c")]),
                s("ctorIdx"),
            ),
        ]),
        let_in(
            "former",
            t_bool(),
            call(
                "Lean.Expr.isSort",
                vec![],
                vec![call(
                    "Lean.Expr.getForallBody",
                    vec![],
                    vec![call("Lean.ConstantInfo.type", vec![], vec![var("info")])],
                )],
            ),
            matching(
                var("info"),
                vec![
                    info_branch("inductInfo", &["inductiveValue"], s("type")),
                    info_branch(
                        "defnInfo",
                        &["definitionValue"],
                        if_then(var("former"), s("type"), call_or_plumbing(lean_owned())),
                    ),
                    info_branch(
                        "opaqueInfo",
                        &["opaqueValue"],
                        call_or_plumbing(lean_owned()),
                    ),
                    info_branch(
                        "axiomInfo",
                        &["axiomValue"],
                        call_or_plumbing(and(lean_owned(), not(var("former")))),
                    ),
                    info_branch("thmInfo", &["theoremValue"], s("plumbing")),
                    info_branch("quotInfo", &["quotientValue"], s("plumbing")),
                    info_branch("ctorInfo", &["constructorValue"], s("plumbing")),
                    info_branch("recInfo", &["recursorValue"], s("plumbing")),
                ],
            ),
        ),
    );
    // a constant is the adapter's own when it is defined in the elaborated
    // file and any component of its name is the adapter's namespace; a private
    // name carries the module's name before it
    let is_own = call(
        "List.elem",
        vec![t_name()],
        vec![
            simple_name("LexLeanExtract"),
            call(
                "Lean.Name.components",
                vec![],
                vec![call("Lean.Name.eraseMacroScopes", vec![], vec![var("n")])],
            ),
        ],
    );
    let fold_constants = |step: Json| {
        call(
            "Lean.SMap.foldStage2",
            vec![t_name(), t_info(), t_set()],
            vec![
                lambda(
                    &[("acc", t_set()), ("n", t_name()), ("info", t_info())],
                    step,
                ),
                empty_set(),
                call("Lean.Environment.constants", vec![], vec![var("env")]),
            ],
        )
    };
    let add_all = |names: Json, acc: Json| {
        call(
            "Array.foldl",
            vec![t_name(), t_set()],
            vec![
                lambda(
                    &[("seen", t_set()), ("used", t_name())],
                    set_insert(var("seen"), var("used")),
                ),
                acc,
                names,
            ],
        )
    };
    let value_uses = |info: Json| {
        matching(
            call(
                "Lean.ConstantInfo.value?",
                vec![],
                vec![info, construct("Bool.true", vec![], vec![])],
            ),
            vec![
                branch(
                    "Option.some",
                    &["value"],
                    call("Lean.Expr.getUsedConstants", vec![], vec![var("value")]),
                ),
                branch("Option.none", &[], array(t_name(), vec![])),
            ],
        )
    };
    let own_names = fold_constants(set_insert(var("acc"), var("n")));
    let own_uses = fold_constants(if_then(
        is_own,
        add_all(
            value_uses(var("info")),
            add_all(
                call(
                    "Lean.Expr.getUsedConstants",
                    vec![],
                    vec![call("Lean.ConstantInfo.type", vec![], vec![var("info")])],
                ),
                var("acc"),
            ),
        ),
        var("acc"),
    ));
    let classify_all = matching(
        var("names"),
        vec![
            branch(
                "List.nil",
                &[],
                pure_in(&command, classed_list.clone(), nil(t_classed())),
            ),
            branch(
                "List.cons",
                &["c", "rest"],
                if_then(
                    set_contains(var("own"), var("c")),
                    call(
                        &own("classifyAll"),
                        vec![],
                        vec![var("env"), var("own"), var("rest")],
                    ),
                    matching(
                        find(var("env"), var("c")),
                        vec![
                            branch(
                                "Option.some",
                                &["info"],
                                bind(
                                    &command,
                                    classed_list.clone(),
                                    classed_list.clone(),
                                    call(
                                        &own("classifyAll"),
                                        vec![],
                                        vec![var("env"), var("own"), var("rest")],
                                    ),
                                    "tail",
                                    pure_in(
                                        &command,
                                        classed_list.clone(),
                                        cons(
                                            t_classed(),
                                            construct(
                                                "Prod.mk",
                                                vec![t_name(), t_string()],
                                                vec![
                                                    var("c"),
                                                    call(
                                                        &own("classify"),
                                                        vec![],
                                                        vec![
                                                            var("env"),
                                                            var("c"),
                                                            var("info"),
                                                        ],
                                                    ),
                                                ],
                                            ),
                                            var("tail"),
                                        ),
                                    ),
                                ),
                            ),
                            branch(
                                "Option.none",
                                &[],
                                throw(
                                    &command,
                                    classed_list.clone(),
                                    cat(vec![
                                        s("lexlean-extract-drift: the adapter uses the unknown constant `"),
                                        name_text(var("c")),
                                        s("`"),
                                    ]),
                                ),
                            ),
                        ],
                    ),
                ),
            ),
        ],
    );
    let adapter_constants = bind(
        &command,
        t_env(),
        classed_list.clone(),
        get_env(&command),
        "env",
        call(
            &own("classifyAll"),
            vec![],
            vec![
                var("env"),
                call(&own("ownNames"), vec![], vec![var("env")]),
                call(
                    &own("sorted"),
                    vec![],
                    vec![call(&own("ownUses"), vec![], vec![var("env")])],
                ),
            ],
        ),
    );
    let list_lines = matching(
        var("used"),
        vec![
            branch("List.nil", &[], pure_in(&command, t_unit(), unit_value())),
            branch(
                "List.cons",
                &["entry", "rest"],
                bind(
                    &command,
                    t_unit(),
                    t_unit(),
                    call(
                        "MonadLiftT.monadLift",
                        vec![ty("IO"), command.clone(), t_unit()],
                        vec![call(
                            "IO.println",
                            vec![t_string()],
                            vec![cat(vec![
                                field_of(var("entry"), "snd"),
                                s(" "),
                                name_text(field_of(var("entry"), "fst")),
                            ])],
                        )],
                    ),
                    "printed",
                    call(&own("listLines"), vec![], vec![var("rest")]),
                ),
            ),
        ],
    );
    vec![
        function(
            &own("classify"),
            &[],
            &[("env", t_env()), ("c", t_name()), ("info", t_info())],
            t_string(),
            classify_body,
            Flags::default(),
            None,
        ),
        function(
            &own("ownNames"),
            &[],
            &[("env", t_env())],
            t_set(),
            own_names,
            Flags::default(),
            None,
        ),
        function(
            &own("ownUses"),
            &[],
            &[("env", t_env())],
            t_set(),
            own_uses,
            Flags::default(),
            None,
        ),
        function(
            &own("classifyAll"),
            &[],
            &[
                ("env", t_env()),
                ("own", t_set()),
                ("names", t_list(t_name())),
            ],
            t_command(classed_list.clone()),
            classify_all,
            PARTIAL,
            None,
        ),
        function(
            &own("adapterConstants"),
            &[],
            &[],
            t_command(classed_list.clone()),
            adapter_constants,
            Flags::default(),
            None,
        ),
        function(
            &own("listLines"),
            &[],
            &[("used", classed_list)],
            t_command(t_unit()),
            list_lines,
            PARTIAL,
            None,
        ),
        function(
            &own("listAdapterConstants"),
            &[],
            &[],
            t_command(t_unit()),
            bind(
                &command,
                t_list(t_classed()),
                t_unit(),
                call(&own("adapterConstants"), vec![], vec![]),
                "used",
                call(&own("listLines"), vec![], vec![var("used")]),
            ),
            Flags::default(),
            None,
        ),
    ]
}

/// The check that the adapter's constants are exactly the registry's, and
/// that each registered type has the constructors the registry lists.
pub fn authority_functions() -> Vec<Json> {
    let command = m_command();
    let problems = t_list(t_string());
    let registered_class = let_in(
        "typeNames",
        t_list(t_name()),
        map_list(
            t_prod(t_name(), t_array(t_string())),
            t_name(),
            "row",
            field_of(var("row"), "fst"),
            call(
                "Array.toList",
                vec![t_prod(t_name(), t_array(t_string()))],
                vec![var("types")],
            ),
        ),
        if_then(
            call(
                "Array.contains",
                vec![t_name()],
                vec![var("calls"), var("c")],
            ),
            s("call"),
            if_then(
                or(
                    call(
                        "List.contains",
                        vec![t_name()],
                        vec![var("typeNames"), var("c")],
                    ),
                    call(
                        "Array.contains",
                        vec![t_name()],
                        vec![var("opaques"), var("c")],
                    ),
                ),
                s("type"),
                if_then(
                    call(
                        "Array.contains",
                        vec![t_name()],
                        vec![var("plumbing"), var("c")],
                    ),
                    s("plumbing"),
                    // a constructor is covered by the row of its type
                    if_then(
                        call(
                            "List.any",
                            vec![t_prod(t_name(), t_array(t_string()))],
                            vec![
                                call(
                                    "Array.toList",
                                    vec![t_prod(t_name(), t_array(t_string()))],
                                    vec![var("types")],
                                ),
                                lambda(
                                    &[("candidate", t_prod(t_name(), t_array(t_string())))],
                                    and(
                                        name_eq(
                                            call("Lean.Name.getPrefix", vec![], vec![var("c")]),
                                            field_of(var("candidate"), "fst"),
                                        ),
                                        call(
                                            "Array.contains",
                                            vec![t_string()],
                                            vec![
                                                field_of(var("candidate"), "snd"),
                                                call(
                                                    "Lean.Name.getString!",
                                                    vec![],
                                                    vec![var("c")],
                                                ),
                                            ],
                                        ),
                                    ),
                                ),
                            ],
                        ),
                        s("plumbing"),
                        s("unregistered"),
                    ),
                ),
            ),
        ),
    );
    let used_problems = matching(
        var("used"),
        vec![
            branch("List.nil", &[], nil(t_string())),
            branch(
                "List.cons",
                &["entry", "rest"],
                let_in(
                    "c",
                    t_name(),
                    field_of(var("entry"), "fst"),
                    let_in(
                        "foreignType",
                        t_bool(),
                        and(
                            string_eq(field_of(var("entry"), "snd"), s("type")),
                            not(call(
                                "Lean.Name.isPrefixOf",
                                vec![],
                                vec![simple_name("Lean"), var("c")],
                            )),
                        ),
                        let_in(
                            "expected",
                            t_string(),
                            if_then(
                                var("foreignType"),
                                s("a type or plumbing"),
                                field_of(var("entry"), "snd"),
                            ),
                            let_in(
                                "registered",
                                t_string(),
                                call(
                                    &own("registeredClass"),
                                    vec![],
                                    vec![
                                        var("calls"),
                                        var("types"),
                                        var("opaques"),
                                        var("plumbing"),
                                        var("c"),
                                    ],
                                ),
                                if_then(
                                    // a constant Lean's namespace does not own, or
                                    // one that is structural, is covered by any row
                                    if_then(
                                        string_eq(field_of(var("entry"), "snd"), s("plumbing")),
                                        not(string_eq(var("registered"), s("unregistered"))),
                                        if_then(
                                            var("foreignType"),
                                            or(
                                                string_eq(var("registered"), s("type")),
                                                string_eq(var("registered"), s("plumbing")),
                                            ),
                                            string_eq(var("registered"), var("expected")),
                                        ),
                                    ),
                                    call(
                                        &own("usedProblems"),
                                        vec![],
                                        vec![
                                            var("calls"),
                                            var("types"),
                                            var("opaques"),
                                            var("plumbing"),
                                            var("rest"),
                                        ],
                                    ),
                                    cons(
                                        t_string(),
                                        cat(vec![
                                            s("`"),
                                            name_text(var("c")),
                                            s("` is used as "),
                                            var("expected"),
                                            s(" but registered as "),
                                            var("registered"),
                                        ]),
                                        call(
                                            &own("usedProblems"),
                                            vec![],
                                            vec![
                                                var("calls"),
                                                var("types"),
                                                var("opaques"),
                                                var("plumbing"),
                                                var("rest"),
                                            ],
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ],
    );
    let unused_problems = matching(
        var("names"),
        vec![
            branch("List.nil", &[], nil(t_string())),
            branch(
                "List.cons",
                &["c", "rest"],
                if_then(
                    call(
                        "List.any",
                        vec![t_classed()],
                        vec![
                            var("used"),
                            lambda(
                                &[("entry", t_classed())],
                                name_eq(field_of(var("entry"), "fst"), var("c")),
                            ),
                        ],
                    ),
                    call(
                        &own("unusedProblems"),
                        vec![],
                        vec![var("used"), var("rest")],
                    ),
                    cons(
                        t_string(),
                        cat(vec![
                            s("`"),
                            name_text(var("c")),
                            s("` is plumbing but unused"),
                        ]),
                        call(
                            &own("unusedProblems"),
                            vec![],
                            vec![var("used"), var("rest")],
                        ),
                    ),
                ),
            ),
        ],
    );
    let actual_constructors = map_list(
        t_name(),
        t_string(),
        "constructorName",
        call("Lean.Name.getString!", vec![], vec![var("constructorName")]),
        field_of(var("inductive"), "ctors"),
    );
    let registered_list = call(
        "Array.toList",
        vec![t_string()],
        vec![field_of(var("row"), "snd")],
    );
    let type_problem = |text: Json| {
        cons(
            t_string(),
            text,
            call(&own("typeProblems"), vec![], vec![var("env"), var("rest")]),
        )
    };
    let continue_types = || call(&own("typeProblems"), vec![], vec![var("env"), var("rest")]);
    let type_problems = matching(
        var("types"),
        vec![
            branch("List.nil", &[], nil(t_string())),
            branch(
                "List.cons",
                &["row", "rest"],
                let_in(
                    "n",
                    t_name(),
                    field_of(var("row"), "fst"),
                    matching(
                        find(var("env"), var("n")),
                        vec![
                            branch(
                                "Option.some",
                                &["info"],
                                matching(
                                    var("info"),
                                    vec![
                                        info_branch(
                                            "inductInfo",
                                            &["inductive"],
                                            if_then(
                                                eq(
                                                    t_list(t_string()),
                                                    actual_constructors,
                                                    registered_list.clone(),
                                                ),
                                                continue_types(),
                                                type_problem(cat(vec![
                                                    s("`"),
                                                    name_text(var("n")),
                                                    s("` has constructors "),
                                                    call(
                                                        "ToString.toString",
                                                        vec![t_list(t_string())],
                                                        vec![map_list(
                                                            t_name(),
                                                            t_string(),
                                                            "listed",
                                                            call(
                                                                "Lean.Name.getString!",
                                                                vec![],
                                                                vec![var("listed")],
                                                            ),
                                                            field_of(var("inductive"), "ctors"),
                                                        )],
                                                    ),
                                                    s(", registered "),
                                                    call(
                                                        "ToString.toString",
                                                        vec![t_list(t_string())],
                                                        vec![registered_list.clone()],
                                                    ),
                                                ])),
                                            ),
                                        ),
                                        otherwise(if_then(
                                            call(
                                                "Array.isEmpty",
                                                vec![t_string()],
                                                vec![field_of(var("row"), "snd")],
                                            ),
                                            continue_types(),
                                            type_problem(cat(vec![
                                                s("`"),
                                                name_text(var("n")),
                                                s("` is not an inductive type, registered with constructors "),
                                                call(
                                                    "ToString.toString",
                                                    vec![t_list(t_string())],
                                                    vec![registered_list.clone()],
                                                ),
                                            ])),
                                        )),
                                    ],
                                ),
                            ),
                            branch(
                                "Option.none",
                                &[],
                                type_problem(cat(vec![
                                    s("`"),
                                    name_text(var("n")),
                                    s("` is not a constant"),
                                ])),
                            ),
                        ],
                    ),
                ),
            ),
        ],
    );
    let check_body = bind(
        &command,
        t_env(),
        t_unit(),
        get_env(&command),
        "env",
        bind(
            &command,
            t_list(t_classed()),
            t_unit(),
            call(&own("adapterConstants"), vec![], vec![]),
            "used",
            let_in(
                "names",
                t_list(t_name()),
                call("Array.toList", vec![t_name()], vec![var("plumbing")]),
                let_in(
                    "differences",
                    problems.clone(),
                    list_append(
                        t_string(),
                        call(
                            &own("usedProblems"),
                            vec![],
                            vec![
                                var("calls"),
                                var("types"),
                                var("opaques"),
                                var("plumbing"),
                                var("used"),
                            ],
                        ),
                        list_append(
                            t_string(),
                            call(
                                &own("unusedProblems"),
                                vec![],
                                vec![var("used"), var("names")],
                            ),
                            call(
                                &own("typeProblems"),
                                vec![],
                                vec![
                                    var("env"),
                                    call(
                                        "Array.toList",
                                        vec![t_prod(t_name(), t_array(t_string()))],
                                        vec![var("types")],
                                    ),
                                ],
                            ),
                        ),
                    ),
                    if_then(
                        call("List.isEmpty", vec![t_string()], vec![var("differences")]),
                        pure_in(&command, t_unit(), unit_value()),
                        throw(
                            &command,
                            t_unit(),
                            cat(vec![
                                s("lexlean-extract-drift: the adapter and its registry differ: "),
                                call(
                                    "String.intercalate",
                                    vec![],
                                    vec![s("; "), var("differences")],
                                ),
                            ]),
                        ),
                    ),
                ),
            ),
        ),
    );
    vec![
        function(
            &own("registeredClass"),
            &[],
            &[
                ("calls", t_array(t_name())),
                ("types", t_registered_types()),
                ("opaques", t_array(t_name())),
                ("plumbing", t_array(t_name())),
                ("c", t_name()),
            ],
            t_string(),
            registered_class,
            Flags::default(),
            None,
        ),
        function(
            &own("usedProblems"),
            &[],
            &[
                ("calls", t_array(t_name())),
                ("types", t_registered_types()),
                ("opaques", t_array(t_name())),
                ("plumbing", t_array(t_name())),
                ("used", t_list(t_classed())),
            ],
            problems.clone(),
            used_problems,
            PARTIAL,
            None,
        ),
        function(
            &own("unusedProblems"),
            &[],
            &[("used", t_list(t_classed())), ("names", t_list(t_name()))],
            problems.clone(),
            unused_problems,
            PARTIAL,
            None,
        ),
        function(
            &own("typeProblems"),
            &[],
            &[
                ("env", t_env()),
                ("types", t_list(t_prod(t_name(), t_array(t_string())))),
            ],
            problems,
            type_problems,
            PARTIAL,
            None,
        ),
        function(
            &own("checkNames"),
            &[],
            &[
                ("calls", t_array(t_name())),
                ("types", t_registered_types()),
                ("opaques", t_array(t_name())),
                ("plumbing", t_array(t_name())),
            ],
            t_command(t_unit()),
            check_body,
            Flags::default(),
            None,
        ),
    ]
}

/// Names written as text, which is all a host can state without Lean's own
/// name syntax: the entry points the driver evaluates take the text of each
/// name and read it as the name it spells.
pub fn entry_functions() -> Vec<Json> {
    let rows = t_array(t_prod(t_string(), t_array(t_string())));
    let named_rows = t_array(t_prod(t_name(), t_array(t_string())));
    let to_names = |texts: Json| {
        call(
            "Array.map",
            vec![t_string(), t_name()],
            vec![
                lambda(
                    &[("text", t_string())],
                    call("String.toName", vec![], vec![var("text")]),
                ),
                texts,
            ],
        )
    };
    vec![
        function(
            &own("toNames"),
            &[],
            &[("texts", t_array(t_string()))],
            t_array(t_name()),
            to_names(var("texts")),
            Flags::default(),
            None,
        ),
        function(
            &own("toTypeRows"),
            &[],
            &[("rows", rows)],
            named_rows,
            call(
                "Array.map",
                vec![
                    t_prod(t_string(), t_array(t_string())),
                    t_prod(t_name(), t_array(t_string())),
                ],
                vec![
                    lambda(
                        &[("row", t_prod(t_string(), t_array(t_string())))],
                        construct(
                            "Prod.mk",
                            vec![t_name(), t_array(t_string())],
                            vec![
                                call("String.toName", vec![], vec![field_of(var("row"), "fst")]),
                                field_of(var("row"), "snd"),
                            ],
                        ),
                    ),
                    var("rows"),
                ],
            ),
            Flags::default(),
            None,
        ),
        function(
            &own("checkAuthority"),
            &[],
            &[
                ("calls", t_array(t_string())),
                ("types", t_array(t_prod(t_string(), t_array(t_string())))),
                ("opaques", t_array(t_string())),
                ("plumbing", t_array(t_string())),
            ],
            t_command(t_unit()),
            call(
                &own("checkNames"),
                vec![],
                vec![
                    call(&own("toNames"), vec![], vec![var("calls")]),
                    call(&own("toTypeRows"), vec![], vec![var("types")]),
                    call(&own("toNames"), vec![], vec![var("opaques")]),
                    call(&own("toNames"), vec![], vec![var("plumbing")]),
                ],
            ),
            Flags::default(),
            None,
        ),
        function(
            &own("main"),
            &[],
            &[
                ("roots", t_array(t_string())),
                ("modules", t_array(t_string())),
            ],
            t_command(t_unit()),
            call(
                &own("mainNames"),
                vec![],
                vec![
                    call(&own("toNames"), vec![], vec![var("roots")]),
                    call(&own("toNames"), vec![], vec![var("modules")]),
                ],
            ),
            Flags::default(),
            None,
        ),
    ]
}

/// The extraction itself: translated definitions, recorded constants with
/// their kernel uses and inductive structure, and the external constants the
/// translated code refers to.
pub fn run_functions() -> Vec<Json> {
    let core = m_core();
    let core_string = t_core(t_string());
    let kernel_uses = |info: Json| {
        call(
            &own("sorted"),
            vec![],
            vec![call(
                "Array.foldl",
                vec![t_name(), t_set()],
                vec![
                    lambda(
                        &[("acc", t_set()), ("used", t_name())],
                        set_insert(var("acc"), var("used")),
                    ),
                    empty_set(),
                    matching(
                        call(
                            "Lean.ConstantInfo.value?",
                            vec![],
                            vec![info, construct("Bool.true", vec![], vec![])],
                        ),
                        vec![
                            branch(
                                "Option.some",
                                &["value"],
                                call("Lean.Expr.getUsedConstants", vec![], vec![var("value")]),
                            ),
                            branch("Option.none", &[], array(t_name(), vec![])),
                        ],
                    ),
                ],
            )],
        )
    };
    let in_project = |n: Json| {
        call(
            &own("inProject"),
            vec![],
            vec![var("env"), var("modules"), n],
        )
    };
    let project_only = |names: Json| {
        call(
            "List.filter",
            vec![t_name()],
            vec![
                lambda(&[("candidate", t_name())], in_project(var("candidate"))),
                names,
            ],
        )
    };
    let other_modules = |names: Json| {
        call(
            "List.filter",
            vec![t_name()],
            vec![
                lambda(&[("outside", t_name())], not(in_project(var("outside")))),
                names,
            ],
        )
    };
    let signature = |decl: Json| {
        call(
            &own("typeConstants"),
            vec![],
            vec![
                field_of(field_of(decl.clone(), "toSignature"), "type"),
                call(
                    &own("paramConstants"),
                    vec![],
                    vec![
                        field_of(field_of(decl, "toSignature"), "params"),
                        empty_set(),
                    ],
                ),
            ],
        )
    };
    let decl_uses = matching(
        field_of(var("decl"), "value"),
        vec![
            branch(
                "Lean.Compiler.LCNF.DeclValue.code",
                &["body"],
                call(
                    &own("codeConstants"),
                    vec![],
                    vec![var("body"), signature(var("decl"))],
                ),
            ),
            branch(
                "Lean.Compiler.LCNF.DeclValue.extern",
                &["attributes"],
                signature(var("decl")),
            ),
        ],
    );
    let uses_list = |uses: Json| {
        arr_list(map_list(
            t_name(),
            t_string(),
            "usedName",
            name_of(var("usedName")),
            call(&own("sorted"), vec![], vec![uses]),
        ))
    };
    let signature_of = |field: &str| field_of(field_of(var("decl"), "toSignature"), field);
    let decl_record = |value: Json, uses: Json| {
        obj(vec![
            ("safe", bool_of(signature_of("safe"))),
            (
                "level_parameters",
                arr_list(map_list(
                    t_name(),
                    t_string(),
                    "levelName",
                    name_of(var("levelName")),
                    signature_of("levelParams"),
                )),
            ),
            (
                "type",
                call(&own("type"), vec![], vec![signature_of("type")]),
            ),
            (
                "parameters",
                arr_list(map_list(
                    t_pure("Param"),
                    t_string(),
                    "parameter",
                    call(&own("param"), vec![], vec![var("parameter")]),
                    call(
                        "Array.toList",
                        vec![t_pure("Param")],
                        vec![signature_of("params")],
                    ),
                )),
            ),
            ("value", value),
            ("uses", uses_list(uses)),
        ])
    };
    let decl_text = let_in(
        "signatureUses",
        t_set(),
        signature(var("decl")),
        matching(
            field_of(var("decl"), "value"),
            vec![
                branch(
                    "Lean.Compiler.LCNF.DeclValue.code",
                    &["body"],
                    decl_record(
                        obj(vec![
                            kind_of("code"),
                            ("code", call(&own("code"), vec![], vec![var("body")])),
                        ]),
                        call(
                            &own("codeConstants"),
                            vec![],
                            vec![var("body"), var("signatureUses")],
                        ),
                    ),
                ),
                branch(
                    "Lean.Compiler.LCNF.DeclValue.extern",
                    &["attributes"],
                    decl_record(obj(vec![kind_of("extern")]), var("signatureUses")),
                ),
            ],
        ),
    );
    let visit_result = |visited: Json, decls: Json, referenced: Json| {
        construct(
            "Prod.mk",
            vec![t_set(), t_prod(t_decls(), t_set())],
            vec![
                visited,
                construct("Prod.mk", vec![t_decls(), t_set()], vec![decls, referenced]),
            ],
        )
    };
    let recurse_code = |queue: Json, visited: Json, decls: Json, referenced: Json| {
        call(
            &own("codeLoop"),
            vec![],
            vec![
                var("env"),
                var("modules"),
                queue,
                visited,
                decls,
                referenced,
            ],
        )
    };
    let translate_decl = bind(
        &core,
        ty_app(
            "Lean.Compiler.LCNF.Decl",
            vec![crate::lcnf_adapter::pure_index()],
        ),
        t_visit(),
        call(
            "Lean.Compiler.LCNF.CompilerM.run",
            vec![t_pure("Decl")],
            vec![call("Lean.Compiler.LCNF.toDecl", vec![], vec![var("n")])],
        ),
        "decl",
        let_in(
            "uses",
            t_list(t_name()),
            call(
                &own("sorted"),
                vec![],
                vec![call(&own("declUses"), vec![], vec![var("decl")])],
            ),
            recurse_code(
                list_append(t_name(), var("rest"), project_only(var("uses"))),
                var("visitedNow"),
                call(
                    "Std.HashMap.insert",
                    vec![t_name(), t_string()],
                    vec![
                        var("decls"),
                        field_of(field_of(var("decl"), "toSignature"), "name"),
                        call(&own("declText"), vec![], vec![var("decl")]),
                    ],
                ),
                call(
                    "List.foldl",
                    vec![t_set(), t_name()],
                    vec![
                        lambda(
                            &[("seen", t_set()), ("outside", t_name())],
                            set_insert(var("seen"), var("outside")),
                        ),
                        var("referenced"),
                        other_modules(var("uses")),
                    ],
                ),
            ),
        ),
    );
    // Lean orders the join points of a function by the numbering of its fresh
    // variables, so a translation that starts wherever the process happens to
    // be gives another program when anything is elaborated before it. Each
    // translation starts from the name generator's initial state, which makes
    // the record a function of the project and the pinned Lean.
    let translate = bind(
        &core,
        t_unit(),
        t_visit(),
        call(
            "Lean.MonadNameGenerator.setNGen",
            vec![core.clone()],
            vec![construct(
                "Lean.NameGenerator.mk",
                vec![],
                vec![simple_name("_uniq"), nat(1)],
            )],
        ),
        "restarted",
        translate_decl,
    );
    let skip = || {
        recurse_code(
            var("rest"),
            var("visitedNow"),
            var("decls"),
            var("referenced"),
        )
    };
    let code_loop = matching(
        var("queue"),
        vec![
            branch(
                "List.nil",
                &[],
                pure_in(
                    &core,
                    t_visit(),
                    visit_result(var("visited"), var("decls"), var("referenced")),
                ),
            ),
            branch(
                "List.cons",
                &["n", "rest"],
                if_then(
                    set_contains(var("visited"), var("n")),
                    recurse_code(var("rest"), var("visited"), var("decls"), var("referenced")),
                    matching(
                        find(var("env"), var("n")),
                        vec![
                            branch(
                                "Option.some",
                                &["info"],
                                let_in(
                                    "visitedNow",
                                    t_set(),
                                    set_insert(var("visited"), var("n")),
                                    if_then(
                                        and(
                                            string_eq(
                                                call(&own("kind"), vec![], vec![var("info")]),
                                                s("definition"),
                                            ),
                                            not(call(
                                                "Lean.isNoncomputable",
                                                vec![],
                                                vec![var("env"), var("n")],
                                            )),
                                        ),
                                        bind(
                                            &core,
                                            t_bool(),
                                            t_visit(),
                                            call(
                                                "Lean.Compiler.LCNF.shouldGenerateCode",
                                                vec![],
                                                vec![var("n")],
                                            ),
                                            "generates",
                                            if_then(var("generates"), translate, skip()),
                                        ),
                                        skip(),
                                    ),
                                ),
                            ),
                            branch("Option.none", &[], throw(&core, t_visit(), unresolved("n"))),
                        ],
                    ),
                ),
            ),
        ],
    );
    let kernel_loop = matching(
        var("queue"),
        vec![
            branch("List.nil", &[], pure_in(&core, t_set(), var("seen"))),
            branch(
                "List.cons",
                &["n", "rest"],
                if_then(
                    set_contains(var("seen"), var("n")),
                    call(
                        &own("kernelLoop"),
                        vec![],
                        vec![var("env"), var("modules"), var("rest"), var("seen")],
                    ),
                    matching(
                        find(var("env"), var("n")),
                        vec![
                            branch(
                                "Option.some",
                                &["info"],
                                call(
                                    &own("kernelLoop"),
                                    vec![],
                                    vec![
                                        var("env"),
                                        var("modules"),
                                        list_append(
                                            t_name(),
                                            var("rest"),
                                            project_only(kernel_uses(var("info"))),
                                        ),
                                        set_insert(var("seen"), var("n")),
                                    ],
                                ),
                            ),
                            branch("Option.none", &[], throw(&core, t_set(), unresolved("n"))),
                        ],
                    ),
                ),
            ),
        ],
    );
    let ctor_row = |ctor_type: Json| {
        obj(vec![
            ("name", name_of(var("c"))),
            (
                "parameters",
                nat_text(field_of(var("constructor"), "numParams")),
            ),
            (
                "fields",
                nat_text(field_of(var("constructor"), "numFields")),
            ),
            ("type", call(&own("type"), vec![], vec![ctor_type])),
        ])
    };
    let not_a_constructor = || {
        throw(
            &core,
            t_list(t_string()),
            cat(vec![
                s("lexlean-extract: `"),
                name_text(var("c")),
                s("` is not a constructor"),
            ]),
        )
    };
    let ctor_rows = matching(
        var("names"),
        vec![
            branch(
                "List.nil",
                &[],
                pure_in(&core, t_list(t_string()), nil(t_string())),
            ),
            branch(
                "List.cons",
                &["c", "rest"],
                matching(
                    find(var("env"), var("c")),
                    vec![
                        branch(
                            "Option.some",
                            &["info"],
                            matching(
                                var("info"),
                                vec![
                                    info_branch(
                                        "ctorInfo",
                                        &["constructor"],
                                        bind(
                                            &core,
                                            t_expr(),
                                            t_list(t_string()),
                                            call(
                                                "Lean.Meta.MetaM.run'",
                                                vec![t_expr()],
                                                vec![call(
                                                    "Lean.Compiler.LCNF.toLCNFType",
                                                    vec![],
                                                    vec![field_of(
                                                        field_of(
                                                            var("constructor"),
                                                            "toConstantVal",
                                                        ),
                                                        "type",
                                                    )],
                                                )],
                                            ),
                                            "constructorType",
                                            bind(
                                                &core,
                                                t_list(t_string()),
                                                t_list(t_string()),
                                                call(
                                                    &own("ctorRows"),
                                                    vec![],
                                                    vec![var("env"), var("rest")],
                                                ),
                                                "tail",
                                                pure_in(
                                                    &core,
                                                    t_list(t_string()),
                                                    cons(
                                                        t_string(),
                                                        ctor_row(var("constructorType")),
                                                        var("tail"),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                    otherwise(not_a_constructor()),
                                ],
                            ),
                        ),
                        branch("Option.none", &[], not_a_constructor()),
                    ],
                ),
            ),
        ],
    );
    let inductive_row = matching(
        find(var("env"), var("n")),
        vec![
            branch(
                "Option.some",
                &["info"],
                matching(
                    var("info"),
                    vec![
                        info_branch(
                            "inductInfo",
                            &["inductive"],
                            bind(
                                &core,
                                t_list(t_string()),
                                t_string(),
                                call(
                                    &own("ctorRows"),
                                    vec![],
                                    vec![var("env"), field_of(var("inductive"), "ctors")],
                                ),
                                "constructors",
                                pure_in(
                                    &core,
                                    t_string(),
                                    obj(vec![
                                        (
                                            "parameters",
                                            nat_text(field_of(var("inductive"), "numParams")),
                                        ),
                                        (
                                            "indices",
                                            nat_text(field_of(var("inductive"), "numIndices")),
                                        ),
                                        ("recursive", bool_of(field_of(var("inductive"), "isRec"))),
                                        ("constructors", arr_list(var("constructors"))),
                                    ]),
                                ),
                            ),
                        ),
                        otherwise(pure_in(&core, t_string(), s("null"))),
                    ],
                ),
            ),
            branch("Option.none", &[], pure_in(&core, t_string(), s("null"))),
        ],
    );
    let row_list =
        |row: Json, extra: Json| call(&own("obj"), vec![], vec![list_append(t_pair(), row, extra)]);
    let constant_rows = matching(
        var("names"),
        vec![
            branch(
                "List.nil",
                &[],
                pure_in(&core, t_list(t_string()), nil(t_string())),
            ),
            branch(
                "List.cons",
                &["n", "rest"],
                matching(
                    find(var("env"), var("n")),
                    vec![
                        branch(
                            "Option.some",
                            &["info"],
                            bind(
                                &core,
                                t_list(t_pair()),
                                t_list(t_string()),
                                call(
                                    &own("facts"),
                                    vec![],
                                    vec![var("env"), var("n"), var("info")],
                                ),
                                "row",
                                bind(
                                    &core,
                                    t_string(),
                                    t_list(t_string()),
                                    call(&own("inductiveRow"), vec![], vec![var("env"), var("n")]),
                                    "inductiveText",
                                    bind(
                                        &core,
                                        t_list(t_string()),
                                        t_list(t_string()),
                                        call(
                                            &own("constantRows"),
                                            vec![],
                                            vec![var("env"), var("decls"), var("rest")],
                                        ),
                                        "tail",
                                        pure_in(
                                            &core,
                                            t_list(t_string()),
                                            cons(
                                                t_string(),
                                                row_list(
                                                    var("row"),
                                                    pairs(vec![
                                                        (
                                                            "kernel_uses",
                                                            arr_list(map_list(
                                                                t_name(),
                                                                t_string(),
                                                                "usedName",
                                                                name_of(var("usedName")),
                                                                kernel_uses(var("info")),
                                                            )),
                                                        ),
                                                        (
                                                            "declaration",
                                                            call(
                                                                "Option.getD",
                                                                vec![t_string()],
                                                                vec![
                                                                    call(
                                                                        "Std.HashMap.get?",
                                                                        vec![t_name(), t_string()],
                                                                        vec![
                                                                            var("decls"),
                                                                            var("n"),
                                                                        ],
                                                                    ),
                                                                    s("null"),
                                                                ],
                                                            ),
                                                        ),
                                                        ("inductive", var("inductiveText")),
                                                    ]),
                                                ),
                                                var("tail"),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        branch(
                            "Option.none",
                            &[],
                            throw(&core, t_list(t_string()), unresolved("n")),
                        ),
                    ],
                ),
            ),
        ],
    );
    let external_rows = matching(
        var("names"),
        vec![
            branch(
                "List.nil",
                &[],
                pure_in(&core, t_list(t_string()), nil(t_string())),
            ),
            branch(
                "List.cons",
                &["r", "rest"],
                matching(
                    find(var("env"), var("r")),
                    vec![
                        branch(
                            "Option.some",
                            &["info"],
                            bind(
                                &core,
                                t_list(t_pair()),
                                t_list(t_string()),
                                call(
                                    &own("facts"),
                                    vec![],
                                    vec![var("env"), var("r"), var("info")],
                                ),
                                "row",
                                bind(
                                    &core,
                                    t_list(t_string()),
                                    t_list(t_string()),
                                    call(
                                        &own("externalRows"),
                                        vec![],
                                        vec![var("env"), var("rest")],
                                    ),
                                    "tail",
                                    let_in(
                                        "compiled",
                                        t_pair(),
                                        pair(
                                            "compiled",
                                            bool_of(is_some(
                                                ty("Lean.IR.Decl"),
                                                call(
                                                    "Lean.IR.findEnvDecl",
                                                    vec![],
                                                    vec![var("env"), var("r")],
                                                ),
                                            )),
                                        ),
                                        pure_in(
                                            &core,
                                            t_list(t_string()),
                                            cons(
                                                t_string(),
                                                row_list(
                                                    var("row"),
                                                    matching(
                                                        var("info"),
                                                        vec![
                                                            info_branch(
                                                                "ctorInfo",
                                                                &["constructor"],
                                                                list(
                                                                    t_pair(),
                                                                    vec![
                                                                        var("compiled"),
                                                                        pair(
                                                                            "inductive_type",
                                                                            name_of(field_of(
                                                                                var("constructor"),
                                                                                "induct",
                                                                            )),
                                                                        ),
                                                                    ],
                                                                ),
                                                            ),
                                                            otherwise(list(
                                                                t_pair(),
                                                                vec![var("compiled")],
                                                            )),
                                                        ],
                                                    ),
                                                ),
                                                var("tail"),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        branch(
                            "Option.none",
                            &[],
                            throw(&core, t_list(t_string()), unresolved("r")),
                        ),
                    ],
                ),
            ),
        ],
    );
    let check_roots = matching(
        var("roots"),
        vec![
            branch("List.nil", &[], pure_in(&core, t_unit(), unit_value())),
            branch(
                "List.cons",
                &["root", "rest"],
                if_then(
                    and(
                        in_project(var("root")),
                        is_some(t_info(), find(var("env"), var("root"))),
                    ),
                    call(
                        &own("checkRoots"),
                        vec![],
                        vec![var("env"), var("modules"), var("rest")],
                    ),
                    throw(
                        &core,
                        t_unit(),
                        cat(vec![
                            s("lexlean-extract: unknown root `"),
                            name_text(var("root")),
                            s("`"),
                        ]),
                    ),
                ),
            ),
        ],
    );
    let to_names = |names: Json| {
        arr_list(map_list(
            t_name(),
            t_string(),
            "listedName",
            name_of(var("listedName")),
            names,
        ))
    };
    let run_body = bind(
        &core,
        t_env(),
        t_string(),
        get_env(&core),
        "env",
        bind(
            &core,
            t_unit(),
            t_string(),
            call(
                &own("checkRoots"),
                vec![],
                vec![
                    var("env"),
                    var("modules"),
                    call("Array.toList", vec![t_name()], vec![var("roots")]),
                ],
            ),
            "checked",
            bind(
                &core,
                t_visit(),
                t_string(),
                call(
                    &own("codeLoop"),
                    vec![],
                    vec![
                        var("env"),
                        var("modules"),
                        call("Array.toList", vec![t_name()], vec![var("roots")]),
                        empty_set(),
                        empty_map(),
                        empty_set(),
                    ],
                ),
                "visit",
                bind(
                    &core,
                    t_set(),
                    t_string(),
                    call(
                        &own("kernelLoop"),
                        vec![],
                        vec![
                            var("env"),
                            var("modules"),
                            call(
                                "Std.HashSet.toList",
                                vec![t_name()],
                                vec![field_of(var("visit"), "fst")],
                            ),
                            empty_set(),
                        ],
                    ),
                    "recorded",
                    bind(
                        &core,
                        t_list(t_string()),
                        t_string(),
                        call(
                            &own("constantRows"),
                            vec![],
                            vec![
                                var("env"),
                                field_of(field_of(var("visit"), "snd"), "fst"),
                                call(&own("sorted"), vec![], vec![var("recorded")]),
                            ],
                        ),
                        "constants",
                        bind(
                            &core,
                            t_list(t_string()),
                            t_string(),
                            call(
                                &own("externalRows"),
                                vec![],
                                vec![
                                    var("env"),
                                    call(
                                        &own("sorted"),
                                        vec![],
                                        vec![field_of(field_of(var("visit"), "snd"), "snd")],
                                    ),
                                ],
                            ),
                            "externals",
                            pure_in(
                                &core,
                                t_string(),
                                obj(vec![
                                    ("spec", str_of(s("lexlean/lcnf-extraction/2"))),
                                    (
                                        "lean",
                                        obj(vec![
                                            (
                                                "version",
                                                str_of(call("Lean.versionString", vec![], vec![])),
                                            ),
                                            (
                                                "githash",
                                                str_of(call("Lean.githash", vec![], vec![])),
                                            ),
                                        ]),
                                    ),
                                    (
                                        "roots",
                                        to_names(call(
                                            "Array.toList",
                                            vec![t_name()],
                                            vec![var("roots")],
                                        )),
                                    ),
                                    ("constants", arr_list(var("constants"))),
                                    ("externals", arr_list(var("externals"))),
                                ]),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    let main_body = call(
        "Lean.Elab.Command.liftCoreM",
        vec![t_unit()],
        vec![bind(
            &core,
            t_string(),
            t_unit(),
            call(&own("run"), vec![], vec![var("roots"), var("modules")]),
            "text",
            call(
                "MonadLiftT.monadLift",
                vec![ty("IO"), core.clone(), t_unit()],
                vec![call("IO.println", vec![t_string()], vec![var("text")])],
            ),
        )],
    );
    let env_params = |extra: Vec<(&'static str, Json)>| {
        let mut parameters = vec![("env", t_env())];
        parameters.extend(extra);
        parameters
    };
    vec![
        function(
            &own("declUses"),
            &[],
            &[("decl", t_pure("Decl"))],
            t_set(),
            decl_uses,
            Flags::default(),
            None,
        ),
        function(
            &own("declText"),
            &[],
            &[("decl", t_pure("Decl"))],
            t_string(),
            decl_text,
            Flags::default(),
            None,
        ),
        function(
            &own("codeLoop"),
            &[],
            &[
                ("env", t_env()),
                ("modules", t_array(t_name())),
                ("queue", t_list(t_name())),
                ("visited", t_set()),
                ("decls", t_decls()),
                ("referenced", t_set()),
            ],
            t_core(t_visit()),
            code_loop,
            PARTIAL,
            None,
        ),
        function(
            &own("kernelLoop"),
            &[],
            &[
                ("env", t_env()),
                ("modules", t_array(t_name())),
                ("queue", t_list(t_name())),
                ("seen", t_set()),
            ],
            t_core(t_set()),
            kernel_loop,
            PARTIAL,
            None,
        ),
        function(
            &own("ctorRows"),
            &[],
            &env_params(vec![("names", t_list(t_name()))]),
            t_core(t_list(t_string())),
            ctor_rows,
            PARTIAL,
            None,
        ),
        function(
            &own("inductiveRow"),
            &[],
            &env_params(vec![("n", t_name())]),
            core_string.clone(),
            inductive_row,
            Flags::default(),
            None,
        ),
        function(
            &own("constantRows"),
            &[],
            &env_params(vec![("decls", t_decls()), ("names", t_list(t_name()))]),
            t_core(t_list(t_string())),
            constant_rows,
            PARTIAL,
            None,
        ),
        function(
            &own("externalRows"),
            &[],
            &env_params(vec![("names", t_list(t_name()))]),
            t_core(t_list(t_string())),
            external_rows,
            PARTIAL,
            None,
        ),
        function(
            &own("checkRoots"),
            &[],
            &env_params(vec![
                ("modules", t_array(t_name())),
                ("roots", t_list(t_name())),
            ]),
            t_core(t_unit()),
            check_roots,
            PARTIAL,
            None,
        ),
        function(
            &own("run"),
            &[],
            &[("roots", t_array(t_name())), ("modules", t_array(t_name()))],
            core_string,
            run_body,
            Flags::default(),
            None,
        ),
        function(
            &own("mainNames"),
            &[],
            &[("roots", t_array(t_name())), ("modules", t_array(t_name()))],
            t_command(t_unit()),
            main_body,
            Flags::default(),
            None,
        ),
    ]
}
