---
name: how-to-verify
description: Use when a reduction rule (or a model's mathematical claim) needs verification before Rust implementation, or when checking reduction-graph topology — orphan problems, NP-hardness reachability from 3-SAT, or whether a proposed rule is dominated by existing paths.
---

# How to Verify

Two jobs: (1) prove a reduction correct before anyone writes Rust, (2) check where a rule or model
sits in the reduction graph. A FAILED verification blocks implementation — report it on the issue
or PR and stop.

## Principles

- The math is verified independently twice (constructor + adversary), then cross-checked.
- Nothing is committed. Proofs and scripts live in the session scratchpad directory (never `/tmp`,
  never the repo). Durable evidence goes in the PR as a certificate comment.
- Everything is variant-qualified (`MinimumDominatingSet/SimpleGraph/One`), never base names only.

## 1. Type-resolution gate (before any math)

Resolve the concrete `Problem::Value` of both endpoints: substitute the rule's concrete generics,
follow aliases and associated types to their `impl`, record the chain with file evidence. If
anything stays unresolved, compile a probe printing
`std::any::type_name::<<Concrete as Problem>::Value>()` from a scratchpad crate with a path
dependency on the repo. Never infer the type from the problem's name or from Python integers.

```text
TYPE RESOLUTION:
  Source: Min<W::Sum>, W = One, <One as WeightElement>::Sum = i64 -> Min<i64>
  Target: Min<usize>
  Full-domain compatibility: FAILED
```

- Witness-compatible: `Or->Or`, `Or->Min`, `Or->Max`, `Min<V>->Min<V>`, `Max<V>->Max<V>` (identical V).
- `Min<S>->Min<T>` with `S != T` is not automatically compatible: proceed only with a declared bound
  covering every legal source instance and a proven total, order-preserving conversion.
- Stop on `Min/Max->Or` (needs a `Decision<P>` source), `Max<->Min` (aggregate or decision
  wrapper), and any `Sum`/`And` target (aggregate only).
- Regression: `MinimumDominatingSet<SimpleGraph, One>` is `Min<i64>`, `MinimumHittingSet` is
  `Min<usize>`. The classical reduction is correct and small cases pass, yet the gate FAILS.

## 2. Typst proof

Theorem, then proof with: numbered _Construction_ (every symbol defined before use); _Correctness_
with genuinely independent (⇒) and (⇐) paragraphs; _Solution extraction_; an overhead table
(target parameter -> formula); a YES example and a NO example (showing why no solution exists),
each with ≥3 variables/vertices and fully worked numbers. Banned: "clearly", "obviously", "it is
easy to see", "straightforward", "similarly for the converse", and any scratch work.

## 3. Constructor script

One Python script, 0 failures, ≥5000 checks (≥10000 for identity and algebraic reductions),
exhaustive over all instances with n ≤ 5 (n ≤ 6 for identity reductions; otherwise ≥300 samples
per (n, m) where full enumeration is infeasible). Seven sections, none empty:

1. symbolic (sympy) check of every overhead formula — "trivial" is no excuse;
2. exhaustive forward + backward: source feasible ⇔ target feasible (optimum preserved);
3. extraction from every feasible target witness (the most skipped section);
4. measured target size vs formula;
5. structural well-formedness of the target (gadget invariants, no degenerate cases);
6. YES example reproduced number-for-number;
7. NO example reproduced, both sides infeasible.

Print a check-count audit per section, and map every claim in the proof to the section that tests
it; add tests for uncovered claims or state why a claim is untestable.

## 4. Adversary

Dispatch a fresh-context subagent given ONLY the Typst proof. It writes its own `reduce`,
`extract_solution`, and source/target feasibility checkers, never importing the constructor;
exhaustive n ≤ 5; `hypothesis` with ≥2 strategies; reproduces both examples; ≥5000 checks. Point
it at the reduction's risk: identity -> all-zero/all-one/alternating configs, n ≤ 6; algebraic ->
case boundaries (e.g. S = 2T, 2T ± 1); gadget -> widget invariants and traversal patterns.

Cross-compare both `reduce` outputs on shared instances: targets structurally identical (up to
documented isomorphism) and feasibility in agreement.

**VERIFIED** only when both scripts pass and cross-comparison agrees. Anything else is **FAILED**,
labelled by cause (constructor bug, adversary bug or ambiguous proof, proof bug, disagreement to
investigate). Never dismiss a disagreement.

## 5. Certificate

When a PR exists (or as soon as it is created), post the evidence:

```bash
python3 scripts/pipeline_pr.py comment --repo "$REPO" --pr "$PR" --body-file <scratchpad>/cert.md
```

`cert.md` starts with `## Verification certificate` and contains: verdict; resolved type chain;
constructor/adversary check counts per section; random seeds and `hypothesis` settings; the n
range and instance families covered; cross-comparison count and disagreements; then both scripts
(and the Typst proof), each collapsed in `<details><summary>…</summary>` code blocks. Without a PR,
report the same content in the conversation.

## Topology checks

```bash
cargo run --example detect_isolated_problems      # problems with no edges in or out
cargo run --example detect_unreachable_from_3sat  # NP-hardness chains from KSatisfiability
pred show <Problem>                               # incoming/outgoing edges per variant
pred path <S/variant> <T/variant> --limit all     # every path with per-step and overall transforms
```

- **Orphans:** list isolated problems with variant counts, and search open issues that would
  connect each (`gh issue list --state open --search "<Name>"`). Meta-issue #610 tracks orphans.
- **NP-hardness:** unreachable NP-hard problems need a new incoming reduction. Problems in P
  (MaximumMatching, 2-SAT, 2-coloring, ...) and intermediate ones (Factoring) are correctly
  unreachable — don't file them. The detectors report base names; confirm the specific variant
  with `pred show` / `pred path`.
- **Redundancy of a proposed rule S -> T:** run `pred path` with variant-qualified endpoints. If no
  path exists, the rule is novel. Otherwise compare each existing path's `Overall:` transform with
  the proposed rule's, parameter by parameter. The rule is dominated if some path is no worse on
  every target parameter (asymptotically, same parameters). `unavailable` overall transforms and
  incomparable expressions (exp/log, different variables) count as not dominated. Report
  **PASS** (not dominated or novel) or **WARN** (name the dominating path with variants), and
  note practical merits a dominated rule may still have (simpler extraction, fewer steps, teaching).
