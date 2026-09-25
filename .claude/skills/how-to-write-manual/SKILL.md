---
name: how-to-write-manual
description: Use when writing, improving, or auditing a problem-def or reduction-rule entry in docs/paper/reductions.typ, or user documentation under docs/src/ (mdBook).
---

# How to Write the Manual

The paper `docs/paper/reductions.typ` is the mathematical manual: one `problem-def` per model, one
`reduction-rule` per directed edge. `docs/src/` is the mdBook user guide (`book.toml` points there).

## Gold standards

- Model: `problem-def("MaximumIndependentSet")` — definition, background with cited algorithms,
  example from the fixture, figure, `pred-commands`.
- Rule: `reduction-rule("MinimumVertexCover", "MaximumIndependentSet"` and its reverse
  `reduction-rule("MaximumIndependentSet", "MinimumVertexCover")` — minimal but complete proof
  structure. For a richer worked example, read `reduction-rule("MaximumIndependentSet", "MaximumClique"`.

## Paper mechanics

- `display-name` dict (near the top) maps `ProblemName` to display text; every problem needs an entry.
- `problem-def(name, variant: none)[def][body]` auto-inserts complexity table, reduction links and
  schema between `def` and `body`; label `<def:Name>`.
- `reduction-rule(source, target, example:, example-source-variant:, example-target-variant:,
  example-caption:, extra:)[statement][proof]` auto-derives overhead from the graph export and
  registers the edge for the completeness check; label `<thm:Source-to-Target>`.
- Every directed reduction needs its own entry, including reverses. Only `Decision<P> <-> P` pairs are
  exempt. The completeness warning block at the end of the paper lists missing edges; your change
  must never increase it.

## Example data — never invent

Examples come only from `docs/paper/data/examples.json`, produced by the canonical example specs:
`canonical_model_example_specs()` beside each model and `canonical_rule_example_specs()` beside each
rule (collected in `src/rules/mod.rs`).

- Models: `#let x = load-model-example("Name")` → keys `problem`, `variant`, `instance`,
  `optimal_config`, `optimal_value`.
- Rules: `#let r = load-example("Src", "Tgt")` → `r.source` / `r.target` (`problem`, `variant`,
  `instance`) and `r.solutions` (each `source_config`, `target_config`).
- Pass `variant:` (models) or `source-variant:` / `target-variant:` (rules, plus the matching
  `example-*-variant:` args on `reduction-rule`) whenever the pair has several fixtures — the loaders
  panic on ambiguity or a missing fixture.
- Pull numbers from the loaded data with Typst expressions; do not hardcode values the fixture holds.
- If the fixture is unsuitable (too big, degenerate, doesn't show the structure), fix the canonical
  spec in code and rebuild — never substitute a hand-made instance.
- Witness semantics: `solutions.at(0)` is the one canonical witness. Never derive "number of optimal
  solutions" from fixture length; if multiplicity matters, argue it from the construction.

Reproducibility block (after the `*Example.*` paragraph for models, at the top of `extra:` for
rules), using the shared helpers only:

```typst
#pred-commands(
  "pred create --example " + problem-spec(x) + " -o x.json",
  "pred solve x.json",
  "pred evaluate x.json --config " + cli-config(x.optimal_config),
)
```

- Models: `problem-spec(x)` renders `Problem/variant/...`; use it rather than a guessed alias —
  canonical fixtures often live on non-default variants. Do not redefine it locally.
- Rules: `"pred create --example " + rule-spec(ex)` (with `ex` from `load-example`) renders
  `Src/k=v/... --to Tgt/k=v/...`, which recreates the rule fixture's source instance, so the
  `--config` from `ex.solutions` applies to it. `problem-spec(ex.source)` or a bare name creates the
  source *model* example instead, a different instance. Keyed tokens avoid ambiguity such as `ILP/i64/bool`.
- `cli-config(...)` renders the JSON `--config`; never `map(str).join(",")`.
- Rules reduce with `pred reduce x.json --via route.json -o bundle.json` (`route.json` = one entry
  selected from `pred path Src Tgt -o paths.json`), then `pred solve bundle.json`. `pred reduce` has no
  `--to`; `target-spec()` and `load-results` do not exist.

## Writing content

**Definitions.** `def` is one self-contained statement: inputs with domains, then objective or
constraint. Every symbol defined before use, in both `def` and proofs.

**Background.** History, applications, notable special cases, and the best known algorithm woven into
prose. Every complexity claim carries `@citation` naming the algorithm. Brute force with nothing
better known → footnote saying so. Unverified reference → `#footnote[Complexity not independently
verified from literature.]`. Keep prose compatible with the auto-generated variant complexity table.

**Rule statement.** Construction summary plus overhead hint in source terms; cite the reduction's
source or add the unverified footnote.

**Proof.** Italic sections `_Construction._`, `_Correctness._` (both directions, $arrow.r.double$ /
$arrow.l.double$), `_Variable mapping._` (only if non-trivial), `_Solution extraction._`.
Reproducible: enough to reimplement. Heavy reductions (roughly 300+ LOC) may sketch the approach and
cite the full construction instead.

**Source priority for math:** the GitHub issue (`gh issue view N`) → derivation documents if
provided → the implementation (`src/rules/<source>_<target>.rs`, ground truth for the construction).
Never invent a proof; if sources disagree with the code, flag it rather than paper over it.

**Worked example.** Show the source instance, walk the construction with concrete numbers and where
each target dimension comes from, verify the canonical witness end to end. Figures use the helpers in
`docs/paper/lib.typ` (`g-node`, `g-edge`, `graph-colors`) — follow the MIS figure.

## Audit checklist

Report each as PASS / WARN / FAIL with a specific reason; read the Rust source before judging math.

Problem-def:
1. `display-name` entry; `def` non-empty and self-contained.
2. Background ≥ 2 informative sentences (applications, history, special cases).
3. Complexity claims cited or footnoted; consistent with `declare_variants!` complexity.
4. `*Example.*` loaded via `load-model-example`, matches `examples.json`, hand-checkable, exercises
   the defining structure, shows the objective/verifier computation.
5. `#figure(` present where the structure is visual; `pred-commands` uses `problem-spec` (models) or `rule-spec` (rules) + `cli-config`.
6. Definition matches `evaluate()` in `src/models/`; no important special case or relation missing.

Reduction-rule:
1. Statement matches what `reduce_to()` builds; prose overhead matches the `#[reduction]` transform.
2. Proof has construction, both correctness directions, extraction; more than a one-liner.
3. Example (`example: true`) loaded via `load-example`, verifies a witness end to end, correct
   `pred-commands` (`--via`, `cli-config`).
4. Reverse edge, if in the graph, has its own entry.
5. Complexity citation or footnote present; no multiplicity claim drawn from fixture length.

Report only issues verifiable from the source; "background is one sentence with no applications"
beats "background is thin".

## Gates

- `make paper` — runs the example/graph/schema exports itself, then compiles the PDF. Must compile
  with no new completeness warnings.
- `make doc` — mdBook plus exports and CLI snippets, for changes under `docs/src/`.
