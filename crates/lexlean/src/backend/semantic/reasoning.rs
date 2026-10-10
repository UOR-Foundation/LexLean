//! Lean lowering of the theorems reasoning declarations generate (§17.12,
//! *Reasoning machines* rule 9): each statement over the fixed
//! `LexLeanReasoning` propositions, and each proof as one fixed template
//! with the declaration's names substituted. No template inspects the
//! goal; Lean's kernel checks every one.

use super::{identifier, latex_line, latex_policy, latex_use, term_uses, tex_escape, Render};
use crate::ir::semantic::reasoning::{FireArm, Formula, GeneratedTheorem, Proof, ProofTerm};
use crate::ir::semantic::{
    MemberRef, ModelUse, ReasoningClaim, ReasoningStrategy, SearchOrder, SemanticDeclaration,
    SemanticModule, SemanticTerm, SemanticType,
};

/// Whether a generated statement mentions the local `name`.
fn formula_uses(formula: &Formula, name: &str) -> bool {
    match formula {
        Formula::Term { term } => term_uses(term, name),
        Formula::Helper { arguments, .. } => {
            arguments.iter().any(|argument| term_uses(argument, name))
        }
        Formula::Implies {
            premise,
            conclusion,
        } => formula_uses(premise, name) || formula_uses(conclusion, name),
        Formula::True => false,
    }
}

impl Render<'_> {
    /// One generated theorem: `public theorem name params : statement :=
    /// proof`.
    pub(super) fn generated_theorem(&self, theorem: &GeneratedTheorem) -> String {
        format!(
            "public theorem {}{}{} : {} :=\n{}",
            identifier(&theorem.name),
            self.type_parameters(&theorem.type_parameters),
            self.parameters(&theorem.parameters),
            self.formula(&theorem.statement),
            self.generated_proof(
                &theorem.proof,
                theorem
                    .parameters
                    .first()
                    .map_or("_", |parameter| parameter.name.as_str()),
                &theorem.type_parameters
            )
        )
    }

    pub(super) fn formula(&self, formula: &Formula) -> String {
        match formula {
            Formula::Term { term } => self.term(term),
            Formula::Helper { helper, arguments } => {
                let mut out = format!("(LexLeanReasoning.{}", helper.lean());
                for argument in arguments {
                    out.push_str(&format!(" ({})", self.term(argument)));
                }
                out.push(')');
                out
            }
            Formula::Implies {
                premise,
                conclusion,
            } => format!(
                "({} -> {})",
                self.formula(premise),
                self.formula(conclusion)
            ),
            Formula::True => "True".to_owned(),
        }
    }

    fn type_argument_list(&self, arguments: &[SemanticType]) -> String {
        arguments
            .iter()
            .map(|argument| format!(" ({})", self.ty(argument)))
            .collect()
    }

    fn proof_term(&self, proof: &ProofTerm) -> String {
        let applied = |head: String, arguments: &[ProofTerm]| {
            if arguments.is_empty() {
                head
            } else {
                let mut out = format!("({head}");
                for argument in arguments {
                    out.push(' ');
                    out.push_str(&self.proof_term(argument));
                }
                out.push(')');
                out
            }
        };
        match proof {
            ProofTerm::Lemma { lemma, arguments } => {
                applied(format!("LexLeanReasoning.{}", lemma.lean()), arguments)
            }
            ProofTerm::Theorem {
                theorem,
                type_arguments,
                arguments,
            } => {
                let head = format!(
                    "{}{}",
                    self.member(theorem),
                    self.type_argument_list(type_arguments)
                );
                if type_arguments.is_empty() {
                    applied(head, arguments)
                } else {
                    applied(format!("({head})"), arguments)
                }
            }
            ProofTerm::Term { term } => format!("({})", self.term(term)),
            ProofTerm::Predicate { binder, body } => format!(
                "(fun ({} : {}) => {})",
                if formula_uses(body, &binder.name) {
                    identifier(&binder.name)
                } else {
                    "_".to_owned()
                },
                self.ty(&binder.r#type),
                self.formula(body)
            ),
            ProofTerm::Assume { binders, body } => {
                format!("(fun {} => {})", binders.join(" "), self.proof_term(body))
            }
            ProofTerm::Hypothesis { name } => name.clone(),
            ProofTerm::Both { left, right } => format!(
                "(And.intro {} {})",
                self.proof_term(left),
                self.proof_term(right)
            ),
            ProofTerm::Infer => "_".to_owned(),
            ProofTerm::Refl => "rfl".to_owned(),
            ProofTerm::Trivial => "True.intro".to_owned(),
        }
    }

    fn arm(&self, arm: &FireArm, target: &[&str]) -> String {
        let binder = if arm.binding { " __b" } else { "" };
        let arguments = if arm.binding {
            format!("__s __b {}", target.join(" "))
        } else {
            format!("__s {}", target.join(" "))
        };
        format!(
            "  | {}{binder} =>\n    exact {}{} {arguments}\n",
            identifier(&arm.constructor),
            self.member(&arm.theorem),
            self.type_argument_list(&arm.type_arguments)
        )
    }

    /// The fixed proof text of one template.
    #[allow(clippy::too_many_lines)]
    fn generated_proof(&self, proof: &Proof, input: &str, type_parameters: &[String]) -> String {
        let x = identifier(input);
        // A member as `dsimp` names it, and as a head applied to arguments:
        // every member a template names is the reasoner's own, so a generic
        // reasoner's are applied to its type parameters.
        let m = |member: &MemberRef| self.member(member);
        let own_arguments: String = type_parameters
            .iter()
            .map(|parameter| format!(" ({})", identifier(parameter)))
            .collect();
        let h = |member: &MemberRef| {
            if own_arguments.is_empty() {
                self.member(member)
            } else {
                format!("({}{own_arguments})", self.member(member))
            }
        };
        match proof {
            Proof::Term { term } => format!("  {}\n", self.proof_term(term)),
            Proof::Unfold { definitions, term } => format!(
                "by\n  dsimp only [{}]\n  exact {}\n",
                definitions.iter().map(m).collect::<Vec<_>>().join(", "),
                self.proof_term(term)
            ),
            Proof::FireCases { arms } => {
                let mut out = "by\n  cases __step with\n".to_owned();
                for arm in arms {
                    out.push_str(&self.arm(arm, &["__t"]));
                }
                out
            }
            Proof::ReplayFire { replay } => format!(
                "by\n  intro llE\n  dsimp only [{}]\n  rw [llE]\n",
                m(replay)
            ),
            Proof::ReplaySound { replay, fire_sound } => format!(
                "by\n  intro llH llT llE\n  cases __acc with\n  | error _ => cases llE\n  | ok llS =>\n    dsimp only [{}] at llE\n    split at llE\n    · cases llE\n    · cases llE\n      exact LexLeanReasoning.Star.tail _ llS _ (llH llS rfl) ({} llS __step _ ‹_›)\n",
                m(replay),
                h(fire_sound)
            ),
            Proof::NextSound { next, fire_sound } => format!(
                "by\n  intro llE\n  dsimp only [{}] at llE\n  split at llE\n  · cases llE\n  · exact {} __s _ __t llE\n",
                m(next),
                h(fire_sound)
            ),
            Proof::NextProgress {
                next,
                fire_progress,
                invariant,
            } => {
                let last = if *invariant {
                    format!("exact fun llJ => {} __s _ __t llJ llE", h(fire_progress))
                } else {
                    format!("exact {} __s _ __t llE", h(fire_progress))
                };
                format!(
                    "by\n  intro llE\n  dsimp only [{}] at llE\n  split at llE\n  · cases llE\n  · {last}\n",
                    m(next)
                )
            }
            Proof::StepNone { step, next } => format!(
                "by\n  intro llE\n  dsimp only [{step}] at llE\n  split at llE\n  · rename_i llH\n    dsimp only [{next}]\n    rw [llH]\n  · rename_i llH\n    split at llE\n    · dsimp only [{next}]\n      rw [llH]\n      assumption\n    · cases llE\n",
                step = m(step),
                next = m(next)
            ),
            Proof::StepSome { step, next } => format!(
                "by\n  intro llE\n  dsimp only [{step}] at llE\n  split at llE\n  · cases llE\n  · rename_i llH\n    split at llE\n    · cases llE\n    · cases llE\n      dsimp only [{next}]\n      rw [llH]\n      assumption\n",
                step = m(step),
                next = m(next)
            ),
            Proof::StepTrace {
                step,
                follow,
                replay_fire,
            } => format!(
                "by\n  intro llE llH\n  dsimp only [{step}] at llE\n  split at llE\n  · cases llE\n  · split at llE\n    · cases llE\n    · cases llE\n      dsimp only [{follow}]\n      rw [LexLeanReasoning.foldSnoc]\n      dsimp only [{follow}] at llH\n      rw [llH]\n      apply {replay_fire}\n      assumption\n",
                step = m(step),
                follow = m(follow),
                replay_fire = h(replay_fire)
            ),
            Proof::StepCount { step } => format!(
                "by\n  intro llE\n  dsimp only [{}] at llE\n  split at llE\n  · cases llE\n  · split at llE\n    · cases llE\n    · cases llE\n      rfl\n",
                m(step)
            ),
            Proof::AcceptSound {
                accept,
                sound,
                type_arguments,
            } => format!(
                "by\n  intro llE\n  dsimp only [{}] at llE\n  split at llE\n  · cases llE\n  · have llC := LexLeanReasoning.checked _ _ _ llE\n    exact llC.right ▸ {}{} _ _ llC.left\n",
                m(accept),
                m(sound),
                self.type_argument_list(type_arguments)
            ),
            Proof::ConcludeSound {
                conclude,
                accept_sound,
                invariant,
            } => format!(
                "by\n  intro {}llE\n  dsimp only [{}] at llE\n  split at llE\n  · cases llE\n    exact {} _ _ _ {}‹_›\n  · split at llE\n    · cases llE\n    · cases llE\n",
                if *invariant { "llJ " } else { "" },
                m(conclude),
                h(accept_sound),
                if *invariant { "llJ " } else { "" }
            ),
            Proof::ConcludeAccept { conclude } => format!(
                "by\n  intro llE\n  dsimp only [{}] at llE\n  split at llE\n  · cases llE\n    assumption\n  · split at llE\n    · cases llE\n    · cases llE\n",
                m(conclude)
            ),
            Proof::ExplainedForward {
                reasoner,
                run,
                answer,
                run_trace,
                conclude_accept,
                conclude_sound,
                run_invariant,
            } => format!(
                "by\n  intro llV llT llE\n  have llTrace := {run_trace} {x}\n{have_invariant}  dsimp only [{reasoner}] at llE\n  generalize llRun : {run} {x} = llR at llE llTrace{at_invariant}\n  split at llE\n  · split at llE\n    · rename_i llW llH\n      cases llE\n      exact And.intro (by dsimp only [{answer}]; rw [llTrace]; exact {conclude_accept} _ _ _ llH) ({conclude_sound} _ _ _ {pass}llH)\n    · cases llE\n  · cases llE\n",
                have_invariant = run_invariant
                    .as_ref()
                    .map_or(String::new(), |theorem| format!("  have llJ := {} {x}\n", h(theorem))),
                at_invariant = if run_invariant.is_some() { " llJ" } else { "" },
                pass = if run_invariant.is_some() { "llJ " } else { "" },
                reasoner = m(reasoner),
                run = h(run),
                answer = m(answer),
                run_trace = h(run_trace),
                conclude_accept = h(conclude_accept),
                conclude_sound = h(conclude_sound)
            ),
            Proof::VerdictForward {
                verdict,
                saturate,
                conclude_sound,
                saturate_invariant,
            } => format!(
                "by\n  intro llV llE\n{have_invariant}  dsimp only [{verdict}] at llE\n  generalize llRun : {saturate} {x} = llR at llE{at_invariant}\n  split at llE\n  · exact {conclude_sound} _ _ _ {pass}llE\n  · cases llE\n",
                have_invariant = saturate_invariant
                    .as_ref()
                    .map_or(String::new(), |theorem| format!("  have llJ := {} {x}\n", h(theorem))),
                at_invariant = if saturate_invariant.is_some() { " llJ" } else { "" },
                pass = if saturate_invariant.is_some() { "llJ " } else { "" },
                verdict = m(verdict),
                saturate = h(saturate),
                conclude_sound = h(conclude_sound)
            ),
            Proof::Extend {
                follow,
                replay_fire,
            } => format!(
                "by\n  intro llH llE\n  dsimp only [{follow}]\n  rw [LexLeanReasoning.foldSnoc]\n  dsimp only [{follow}] at llH\n  rw [llH]\n  exact {replay_fire} _ __step __t llE\n",
                follow = m(follow),
                replay_fire = h(replay_fire)
            ),
            Proof::SuccessorsFree { successors, extend } => format!(
                "by\n  intro llH\n  dsimp only [{}]\n  split\n  · exact LexLeanReasoning.All.nil\n  · exact LexLeanReasoning.allSingle _ _ ({} _ __node _ _ llH ‹_›)\n",
                m(successors),
                h(extend)
            ),
            Proof::SuccessorsBound {
                successors,
                collect,
                extend,
                follow,
                node,
            } => format!(
                "by\n  intro llH\n  dsimp only [{successors}]\n  exact LexLeanReasoning.foldInvariant _ (LexLeanReasoning.All (fun (llN : {node}) => {follow} _ llN.trace = Except.ok llN.state))\n    (fun llA llB llP => by\n      dsimp only [{collect}]\n      split\n      · exact llP\n      · exact LexLeanReasoning.allAppend _ _ _ llP (LexLeanReasoning.allSingle _ _ ({extend} _ __node _ _ llH ‹_›)))\n    _ _ LexLeanReasoning.All.nil\n",
                successors = m(successors),
                collect = m(collect),
                extend = h(extend),
                follow = h(follow),
                node = self.ty(node)
            ),
            Proof::SearchStep {
                search_step,
                successors_ok,
                fresh_ok,
                order,
            } => {
                let successors =
                    format!("({} _ llNode llNodeOk)", h(successors_ok));
                let fresh = match fresh_ok {
                    Some(fresh_ok) => format!("({} _ _ _ {successors})", h(fresh_ok)),
                    None => successors,
                };
                let ordered = match order {
                    SearchOrder::BreadthFirst => {
                        format!("(LexLeanReasoning.allAppend _ _ _ llRestOk {fresh})")
                    }
                    SearchOrder::DepthFirst => {
                        format!("(LexLeanReasoning.allAppend _ _ _ {fresh} llRestOk)")
                    }
                };
                format!(
                    "by\n  intro llE llH\n  dsimp only [{}] at llE\n  split at llE\n  · cases llE\n  · split at llE\n    · cases llE\n    · rename_i llNode llRest llFr\n      have llAll := And.left llH\n      rw [llFr] at llAll\n      have llNodeOk := LexLeanReasoning.allHead _ llNode llRest llAll\n      have llRestOk := LexLeanReasoning.allTail _ llNode llRest llAll\n      split at llE\n      · rename_i llV llAcc\n        cases llE\n        exact And.intro llRestOk (And.intro llNodeOk llAcc)\n      · cases llE\n        exact And.intro (LexLeanReasoning.capAll _ _ _ {ordered}) True.intro\n",
                    m(search_step)
                )
            }
            Proof::SearchPeak { search_step } => format!(
                "by\n  intro llE llH\n  dsimp only [{}] at llE\n  split at llE\n  · cases llE\n  · split at llE\n    · cases llE\n    · split at llE\n      · cases llE\n        exact llH\n      · cases llE\n        exact LexLeanReasoning.peakBound _ _ _ llH (LexLeanReasoning.capBound _ _)\n",
                m(search_step)
            ),
            Proof::SearchCount { search_step } => format!(
                "by\n  intro llE\n  dsimp only [{}] at llE\n  split at llE\n  · cases llE\n  · split at llE\n    · cases llE\n    · split at llE\n      · cases llE\n        rfl\n      · cases llE\n        rfl\n",
                m(search_step)
            ),
            Proof::SearchGrowth { search_step } => format!(
                "by\n  intro llE\n  dsimp only [{}] at llE\n  split at llE\n  · cases llE\n  · split at llE\n    · cases llE\n    · split at llE\n      · cases llE\n        first | exact Nat.le_refl _ | exact Nat.le_succ _\n      · cases llE\n        first | exact Nat.le_refl _ | exact Nat.le_succ _\n",
                m(search_step)
            ),
            Proof::ExplainedSearch {
                reasoner,
                run,
                answer,
                search_ok,
                accept_sound,
                follow_invariant,
            } => format!(
                "by\n  intro llV llT llE\n  have llSearch := {search_ok} {x}\n  dsimp only [{reasoner}] at llE\n  generalize llRun : {run} {x} = llR at llE llSearch\n  split at llE\n  · rename_i llHit llF\n    cases llE\n    have llOk := And.right llSearch\n    rw [llF] at llOk\n    exact And.intro (by dsimp only [{answer}]; rw [And.left llOk]; exact And.right llOk) ({accept_sound} _ _ _ {pass}(And.right llOk))\n  · cases llE\n",
                pass = follow_invariant.as_ref().map_or(String::new(), |theorem| format!("({} {x} _ _ (And.left llOk)) ", h(theorem))),
                reasoner = m(reasoner),
                run = h(run),
                answer = m(answer),
                search_ok = h(search_ok),
                accept_sound = h(accept_sound)
            ),
            Proof::VerdictSearch {
                reasoner,
                verdict,
                explained,
            } => format!(
                "by\n  intro llV llE\n  dsimp only [{verdict}] at llE\n  generalize llHP : {reasoner} {x} = llR at llE\n  cases llR with\n  | error _ => cases llE\n  | ok llP =>\n    cases llE\n    exact And.right ({explained} _ llP.1 llP.2 llHP)\n",
                reasoner = h(reasoner),
                verdict = m(verdict),
                explained = h(explained)
            ),
            Proof::RunInvariant {
                run_state,
                saturate_invariant,
            } => format!(
                "by\n  rw [And.left ({} {x})]\n  exact {} {x}\n",
                h(run_state),
                h(saturate_invariant)
            ),
            Proof::VerifySound {
                verify,
                sound,
                type_arguments,
            } => format!(
                "by\n  intro llE\n  dsimp only [{verify}] at llE\n  have llC := LexLeanReasoning.checked _ _ _ llE\n  cases llC.right\n  exact And.intro (by dsimp only [{verify}]; rw [llC.left]; rfl) ({sound}{arguments} _ _ llC.left)\n",
                verify = m(verify),
                sound = m(sound),
                arguments = self.type_argument_list(type_arguments)
            ),
            Proof::TrySound {
                attempt,
                verify_sound,
            } => format!(
                "by\n  intro llH llV llE\n  dsimp only [{attempt}] at llE\n  split at llE\n  · exact llH llV llE\n  · split at llE\n    · exact {verify_sound} _ _ _ llE\n    · cases llE\n",
                attempt = m(attempt),
                verify_sound = h(verify_sound)
            ),
            Proof::TryCount { attempt } => format!(
                "by\n  intro llH\n  dsimp only [{}]\n  split\n  · exact llH\n  · split\n    · rename_i llB\n      exact LexLeanReasoning.bltSucc _ _ llB\n    · exact llH\n",
                m(attempt)
            ),
            Proof::VerdictGenerate {
                verdict,
                run,
                run_sound,
            } => format!(
                "by\n  intro llV llE\n  have llS := {run_sound} {x}\n  dsimp only [{verdict}] at llE\n  generalize llRun : {run} {x} = llR at llE llS\n  split at llE\n  · rename_i llHit llF\n    cases llE\n    exact And.right (llS _ llF)\n  · cases llE\n",
                verdict = m(verdict),
                run = h(run),
                run_sound = h(run_sound)
            ),
            Proof::ExplainedGenerate {
                reasoner,
                run,
                run_sound,
                answer,
            } => format!(
                "by\n  intro llV llT llE\n  have llS := {run_sound} {x}\n  dsimp only [{reasoner}] at llE\n  generalize llRun : {run} {x} = llR at llE llS\n  split at llE\n  · rename_i llHit llF\n    cases llE\n    have llOk := llS _ llF\n    exact And.intro (by dsimp only [{answer}]; exact And.left llOk) (And.right llOk)\n  · cases llE\n",
                reasoner = m(reasoner),
                run = h(run),
                run_sound = h(run_sound),
                answer = m(answer)
            ),
        }
    }
}

/// Language 1.2 (reasoning): what a reasoning declaration states, then
/// exactly what linking elaborated it to, its generated obligations with
/// the theorems that state them, and the theorems it generates. The
/// document lists no trace value and never calls anything verified:
/// verification is the attestation's (§22.9).
#[allow(clippy::too_many_lines)]
pub(super) fn latex_reasoning(
    render: &Render<'_>,
    module: &SemanticModule,
    index: usize,
    declaration: &SemanticDeclaration,
    text: &mut String,
) {
    let uses = |references: &[ModelUse]| {
        references
            .iter()
            .map(|reference| latex_use(render, reference))
            .collect::<Vec<_>>()
            .join(", ")
    };
    match declaration {
        SemanticDeclaration::Logic {
            state,
            relation,
            invariant,
            ranking,
            axioms,
            ..
        } => {
            latex_line(
                text,
                "States",
                &format!(
                    "{}, {} : {}",
                    state.name,
                    state.next,
                    render.ty(&state.r#type)
                ),
            );
            latex_line(text, "Relation", &render.member(relation));
            if let Some(invariant) = invariant {
                latex_line(
                    text,
                    "Invariant",
                    &format!(
                        "{}, preserved by the relation as {} states",
                        render.member(&invariant.predicate),
                        render.member(&invariant.preserves)
                    ),
                );
            }
            if let Some(ranking) = ranking {
                latex_line(text, "Ranking", &render.member(ranking));
            }
            latex_line(text, "Axiom policy", &latex_policy(axioms));
        }
        SemanticDeclaration::InferenceRule {
            logic,
            binding,
            guard,
            conclusion,
            soundness,
            progress,
            executable,
            axioms,
            ..
        } => {
            latex_line(text, "Logic", &latex_use(render, logic));
            if let Some(binding) = binding {
                latex_line(
                    text,
                    "Binding",
                    &format!(
                        "{} : {}, ranging over {} in order",
                        binding.name,
                        render.ty(&binding.r#type),
                        render.term(&binding.candidates)
                    ),
                );
            }
            latex_line(text, "Guard", &render.term(guard));
            latex_line(text, "Conclusion", &render.term(conclusion));
            latex_line(text, "Soundness", &render.member(soundness));
            if let Some(progress) = progress {
                latex_line(text, "Progress", &render.member(progress));
            }
            latex_line(
                text,
                "Execution",
                if *executable {
                    "executable, applied only through its guard"
                } else {
                    "formal"
                },
            );
            latex_line(text, "Axiom policy", &latex_policy(axioms));
        }
        SemanticDeclaration::Verifier {
            subject,
            candidate,
            specification,
            check,
            sound,
            complete,
            axioms,
            ..
        } => {
            latex_line(
                text,
                "Subject",
                &format!("{} : {}", subject.name, render.ty(&subject.r#type)),
            );
            latex_line(
                text,
                "Candidate",
                &format!("{} : {}", candidate.name, render.ty(&candidate.r#type)),
            );
            latex_line(text, "Specification", &render.member(specification));
            latex_line(text, "Check", &render.member(check));
            latex_line(text, "Soundness", &render.member(sound));
            if let Some(complete) = complete {
                latex_line(text, "Completeness", &render.member(complete));
            }
            latex_line(text, "Axiom policy", &latex_policy(axioms));
        }
        SemanticDeclaration::Reasoner {
            type_parameters,
            logic,
            observation,
            observe,
            rules,
            strategy,
            answer,
            verifier,
            claims,
            executable,
            axioms,
            ..
        } => {
            if !type_parameters.is_empty() {
                latex_line(text, "Type parameters", &type_parameters.join(", "));
            }
            if let Some(logic) = logic {
                latex_line(text, "Logic", &latex_use(render, logic));
            }
            latex_line(
                text,
                "Observation",
                &match observe {
                    Some(observe) => format!(
                        "{} : {}, observed as {}",
                        observation.name,
                        render.ty(&observation.r#type),
                        render.term(observe)
                    ),
                    None => format!("{} : {}", observation.name, render.ty(&observation.r#type)),
                },
            );
            if !rules.is_empty() {
                latex_line(text, "Rules in priority order", &uses(rules));
            }
            let bound = |term: &Option<SemanticTerm>| {
                term.as_ref()
                    .map_or_else(|| "none".to_owned(), |term| render.term(term))
            };
            let strategy_text = match strategy {
                ReasoningStrategy::Forward { fuel } => {
                    format!("forward, at most {} iterations", bound(fuel))
                }
                ReasoningStrategy::Search {
                    order,
                    fuel,
                    frontier,
                    deduplicate,
                } => format!(
                    "{} search, at most {} iterations and {} frontier nodes{}",
                    match order {
                        SearchOrder::BreadthFirst => "breadth-first",
                        SearchOrder::DepthFirst => "depth-first",
                    },
                    bound(fuel),
                    bound(frontier),
                    if *deduplicate {
                        ", each state expanded once"
                    } else {
                        ""
                    }
                ),
                ReasoningStrategy::GenerateAndVerify { generator, budget } => format!(
                    "generate and verify the candidates {}, checking at most {}",
                    render.term(generator),
                    bound(budget)
                ),
            };
            latex_line(text, "Strategy", &strategy_text);
            if let Some(answer) = answer {
                latex_line(
                    text,
                    "Answer",
                    &format!(
                        "{} |-> {} : Option ({})",
                        answer.name,
                        render.term(&answer.value),
                        render.ty(&answer.r#type)
                    ),
                );
            }
            latex_line(text, "Verifier", &latex_use(render, verifier));
            for claim in claims {
                let (kind, theorem) = match claim {
                    ReasoningClaim::InitialInvariant { theorem } => ("initial invariant", theorem),
                    ReasoningClaim::Terminates { theorem } => ("terminates", theorem),
                    ReasoningClaim::AnswerCorrect { theorem } => {
                        ("answer correct, its check erased", theorem)
                    }
                    ReasoningClaim::ObservationInvariant {
                        predicate,
                        initial,
                        preserved,
                    } => {
                        latex_line(
                            text,
                            "Claim",
                            &format!(
                                "observation invariant {}, discharged by {} and {}",
                                render.member(predicate),
                                render.member(initial),
                                render.member(preserved)
                            ),
                        );
                        continue;
                    }
                };
                latex_line(
                    text,
                    "Claim",
                    &format!("{kind}, discharged by {}", render.member(theorem)),
                );
            }
            latex_line(
                text,
                "Execution",
                if *executable {
                    "executable through its verifier"
                } else {
                    "formal"
                },
            );
            latex_line(text, "Axiom policy", &latex_policy(axioms));
        }
        SemanticDeclaration::Structure { .. }
        | SemanticDeclaration::Class { .. }
        | SemanticDeclaration::Instance { .. }
        | SemanticDeclaration::Inductive { .. }
        | SemanticDeclaration::Definition { .. }
        | SemanticDeclaration::Theorem { .. }
        | SemanticDeclaration::Artifact { .. }
        | SemanticDeclaration::Contract { .. }
        | SemanticDeclaration::Realization { .. }
        | SemanticDeclaration::Evidence { .. }
        | SemanticDeclaration::Model { .. } => {}
    }
    for obligation in module.elaboration.obligations(index) {
        let binders = if obligation.parameters.is_empty() {
            String::new()
        } else {
            format!("forall{}, ", render.parameters(&obligation.parameters))
        };
        latex_line(
            text,
            &format!(
                "Obligation ({})",
                tex_escape(&obligation.role.replace('`', ""))
            ),
            &format!(
                "{binders}{}; stated exactly by {}",
                render.term(&obligation.statement),
                render.member(&obligation.theorem)
            ),
        );
    }
    for derived in module.elaboration.lowered(index) {
        let shown = match derived {
            SemanticDeclaration::Definition {
                name,
                parameters,
                result,
                body,
                ..
            } => format!(
                "{name}{} : {} := {}",
                render.parameters(parameters),
                render.ty(result),
                render.term(body)
            ),
            SemanticDeclaration::Inductive {
                name, constructors, ..
            } => format!(
                "inductive {name} with {}",
                constructors
                    .iter()
                    .map(|constructor| {
                        let fields = constructor
                            .fields
                            .iter()
                            .map(|field| render.ty(field))
                            .collect::<Vec<_>>()
                            .join(", ");
                        format!("{}({fields})", constructor.name)
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            SemanticDeclaration::Structure { name, fields, .. } => format!(
                "structure {name} with {}",
                fields
                    .iter()
                    .map(|field| format!("{} : {}", field.name, render.ty(&field.r#type)))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            other => other.name().to_owned(),
        };
        latex_line(text, "Elaborates to", &shown);
    }
    for theorem in module.elaboration.theorems(index) {
        let binders = if theorem.parameters.is_empty() {
            String::new()
        } else {
            format!("forall{}, ", render.parameters(&theorem.parameters))
        };
        latex_line(
            text,
            &format!(
                "Generates theorem ({} template)",
                theorem.template().replace('_', " ")
            ),
            &format!(
                "{} : {binders}{}",
                theorem.name,
                render.formula(&theorem.statement)
            ),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::formula_uses;
    use crate::ir::semantic::reasoning::Formula;
    use crate::ir::semantic::SemanticTerm;

    /// A predicate binder lowers as `_` exactly when its body never names
    /// it, so the linter that verification treats as an error stays quiet.
    #[test]
    fn formula_uses_sees_every_term() {
        let x = SemanticTerm::Var {
            name: "x".to_owned(),
        };
        let formula = Formula::Implies {
            premise: Box::new(Formula::True),
            conclusion: Box::new(Formula::Term { term: x }),
        };
        assert!(formula_uses(&formula, "x"));
        assert!(!formula_uses(&formula, "y"));
    }
}
