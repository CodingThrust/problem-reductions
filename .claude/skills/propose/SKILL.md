---
name: propose
description: Use when someone wants to propose a new problem model or reduction rule for the library — interviews them in mathematical language, fills gaps from the reduction graph and literature, checks the draft, and files `[Model]` / `[Rule]` GitHub issues
---

# Propose a Model or Rule

Turn a domain expert's idea into GitHub issues that pass `how-to-triage-issue` on the
first try. The proposer may know no Rust; speak mathematics only.

Invocation: `/propose`, `/propose model`, `/propose rule`.

<HARD-GATE>
No code, no repo edits, no implementation skills. The only repo write allowed is downloading
reference PDFs into `docs/research/raw/`. The output is GitHub issues filed with `gh issue create`,
and only after the user explicitly approves the final drafts.
</HARD-GATE>

## What a finished proposal contains

The issue must fill every section of `.github/ISSUE_TEMPLATE/problem.md` (model) or
`.github/ISSUE_TEMPLATE/rule.md` (rule). Read the template before drafting. Quality bars:

- **Reference read, not just cited.** Download the main reference PDF to `docs/research/raw/`
  and read the actual theorem, construction, and proof. Name the exact theorem/section in the
  issue. A rule's algorithm section must be implementable (notation, gadgets, constraint
  families, parameter choices, solution extraction); a citation-only algorithm fails.
  Flag ambiguities or transcription risks in the source instead of smoothing them over. If no
  full text is obtainable, say so and mark the proposal unverified.
- **Bug-catching example.** Small enough to verify by hand, but with constraints that interact
  and are tight, so a wrong implementation gives a wrong answer. No triangles, all-zeros, or
  single-element degenerate cases. Models: optimization gives an optimal witness and value;
  feasibility gives a YES witness with justification plus a NO instance with an infeasibility
  argument. Rules: fully worked source → construction → target, with optimal vs suboptimal (or
  YES vs NO) behavior visible. Build the witnesses yourself; never ask the proposer for them.
- **No orphans.** A model needs at least one companion rule issue connecting it to the graph.
  An orphan model gets rejected in review; if the proposer insists on skipping, put a visible
  warning in the Reduction Rule Crossref section.
- **ILP claim needs its rule.** Check "solvable by reducing directly to ILP" only when a direct
  `[Rule] <Model> to ILP` companion issue is filed in this same session. Checking it means the
  model and that rule ship together in one PR. Otherwise the model is brute-force only; never
  mention ILP/QUBO under "How to solve" without a concrete issue number.
- **Concrete complexity.** Best known exact algorithm with numeric bases/exponents
  (`1.1996^n`, not `(2-ε)^n`, not `2^(ω n/3)`), author, year, link, and BibTeX. Polynomial
  problems must not be given exponential bounds.
- **Math language, not implementation types.** Fields are "a graph", "nonnegative integer
  weights", "a list of subsets of {0,…,n−1}". Do not ask the proposer to pick Rust types,
  integer widths, or trait names; implementers derive those. Variables are described as a
  configuration vector: count, per-variable domain, meaning.
- **Every symbol defined before use.** Size-overhead formulas use names from the target's
  `parameters` (`pred show <target> --json`), and source parameters on the right-hand side.
- **BibTeX** for each reference at the end of the issue.

## Infer silently, ask only for gaps

Before asking anything, gather what the tools already know. Build `pred` if missing
(`command -v pred || make cli`; it takes minutes, so don't rebuild when present).

```bash
pred list --json                      # catalog; `pred list <query>` to search names/aliases
pred list --rules --all               # every registered rule
pred show <Problem> --json            # fields (schema/inputs), parameters, complexity, reduces_to/from
pred to <Problem> --hops 2 --json     # incoming neighbors
pred from <Problem> --hops 2 --json   # outgoing neighbors
pred path <Src> <Tgt> --limit 5       # set of existing paths (not a single "best" one)
gh issue list --label model --state all --limit 500 --json number,title,state
gh issue list --label rule  --state all --limit 500 --json number,title,state
cargo run --example detect_isolated_problems      # orphan problems
cargo run --example detect_unreachable_from_3sat  # problems lacking an NP-hardness chain
```

Read one closed issue of the same kind and domain to match the expected level of detail.
Use WebSearch for literature; for references already in `docs/paper/references.bib`,
`python3 scripts/fetch_papers.py lookup|download|scihub` fetches PDFs, otherwise
`curl -L '<pdf-url>' -o docs/research/raw/<author-year-short-title>.pdf` and check with `file`.

Then interview **one question at a time**, each with a recommended answer the user can accept.
State inferred facts as one-line confirmations ("This is a minimization over binary vertex
choices on a general graph — right?") instead of multiple-choice menus. Pre-fill textbook
reductions (Garey & Johnson, Karp) from the literature, but still let the user confirm the
algorithm, correctness argument, overhead, and example before drafting.

## Model proposals

1. Get a name or rough description. Check for overlap: if `pred show` finds the same problem
   or a restriction/generalization, say so and let the user choose between a new variant,
   rules for the existing problem instead, or both. Mention if the existing one is an orphan.
2. Fill definition, variables, input data, and variants (graph topologies, weighted or not,
   fixed vs general K) only as far as they are meaningful for this problem.
3. Complexity and reference: offer up to three literature candidates with links, read the
   chosen paper, fetch its BibTeX.
4. Solving: brute force is the baseline, never a question. Ask about a direct ILP claim only
   when a natural linear formulation exists (see the ILP bar above). Mention a polynomial-time
   algorithm if one is known.
5. Example and expected outcome: propose about three candidates of different sizes; the user
   picks or asks for a new batch.
6. Companion rules: rank candidates as in the rule priorities below, with `<Model> → ILP` on
   top when the model has no outgoing edge and admits a linear formulation, and mandatory when
   the ILP claim is made. Brainstorm each chosen rule with the rule flow, lighter on context.

## Rule proposals

Rank candidate pairs from the topology data before asking which pair the user wants. Both
endpoints must exist (or be proposed in this session). Priorities:

1. Connect an orphan from `detect_isolated_problems`.
2. Fill an NP-hardness gap from `detect_unreachable_from_3sat` (a reduction *from* a problem
   already reachable from 3-SAT).
3. Give a leaf problem (empty `pred from`) a path to ILP via `<Leaf> → ILP`.
4. Connect to a large cluster (QUBO, ILP, SAT families).

Exclude pairs that are already registered and pairs with an existing open or closed issue —
point the user to that issue instead. Also run `pred path <Src> <Tgt>`: if paths already exist,
the rule must justify itself (better overhead, NP-hardness direction, simpler witness mapping).

Then, for the chosen pair, study both endpoints with `pred show --json` (fields, parameters,
optimization vs feasibility, value domains such as big integers vs bounded integers), read the
main reference, and settle with the user: motivation, algorithm, correctness argument
(feasibility or optimality preserved), size overhead, validation method, worked example.

## Check, approve, file

1. Draft every issue in full and run the `how-to-triage-issue` checks against the drafts:
   usefulness (`pred show <NewModel>` fails; for rules, no redundant existing path), non-
   triviality (not a renaming, subtype coercion, or variable substitution), correctness
   (references exist in `docs/paper/references.bib` or verifiably online; claims match the
   PDF), completeness (all template sections), writing (symbols consistent, overhead names
   match target `parameters`, example fully worked), ILP-claim consistency. Fix what you can;
   ask for the rest.
2. Show the drafts and get explicit approval ("file it"). Revisions loop back to step 1.
3. File the model first, then its rules, then patch the model's crossref with real numbers:

```bash
gh issue create --title "[Model] <Name>" --label model --body-file <draft>
gh issue create --title "[Rule] <Source> to <Target>" --label rule --body-file <draft>   # mention #<model>
gh issue edit <model-number> --body-file <updated-draft>   # real rule issue numbers in crossref / How to solve
```

Keep draft files in the scratchpad, not the repo. Print all issue URLs at the end.
