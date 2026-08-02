# NAESatisfiability to Satisfiability Reduction

Implement issue #1101 as a witness-preserving reduction from arbitrary-width
`NAESatisfiability` to `Satisfiability`. The construction retains every source
variable and replaces each NAE clause `C` by the two SAT clauses `C` and its
literalwise complement. Target witnesses therefore map back by identity.

## Batch 1: Verify and implement the rule

Follow `.agents/skills/add-rule/SKILL.md` Steps 0-5 and 7, including the default
`.agents/skills/verify-reduction/SKILL.md` procedure before writing Rust code.

1. Confirm the `Or -> Or` type pairing in
   `src/models/formula/nae_satisfiability.rs` and `src/models/formula/sat.rs`.
2. Verify the construction mathematically with ephemeral artifacts:
   - Write a self-contained Typst proof covering arbitrary clause width,
     repeated literals and variables, tautological clauses, and the empty
     conjunction.
   - Independently exercise the clause-doubling constructor, identity solution
     extraction, exact overhead (`num_vars`, `2 * num_clauses`,
     `2 * num_literals`), target validity, and both directions on at least 5,000
     checks, including exhaustive assignments through five variables.
   - Use a three-variable feasible example and a three-variable infeasible
     example, then cross-compare an independent adversary implementation.
3. Add `src/rules/naesatisfiability_satisfiability.rs` with one
   `#[reduction]` registration, a direct target construction, and identity
   extraction. Do not add adapters or alternate implementations.
4. Register the module in `src/rules/mod.rs`.
5. Add focused tests in
   `src/unit_tests/rules/naesatisfiability_satisfiability.rs` for:
   - the issue's three-clause closed-loop witness;
   - an infeasible repeated-literal formula;
   - exact doubled clauses and all three overhead metrics;
   - empty conjunction, arbitrary width, repeated literals, tautological
     clauses, and identity extraction;
   - direct reduction-graph registration and canonical example integrity.
6. Add the issue's three-variable instance as the canonical example in the
   rule's `canonical_rule_example_specs()` registration, following the current
   module-local example-db pattern.
7. Run focused Rust tests and regenerate graph/schema exports and example
   fixtures needed by the paper. Keep only tracked artifacts required by the
   rule.

## Batch 2: Document the verified rule

With fresh context, follow `.agents/skills/add-rule/SKILL.md` Step 6.

1. Add the Gurumukhani-Paturi-Saks-Talebanfard STACS 2025 reference to
   `docs/paper/references.bib` if it is not already present; retain Schaefer's
   classical citation where useful.
2. Add the `NAESatisfiability -> Satisfiability` theorem, proof, extraction,
   and tutorial example to `docs/paper/reductions.typ`. Derive the command block
   and concrete values from the canonical example fixture.
3. State the two-clause construction and prove both directions independently:
   the original clause supplies a true literal, while its complement supplies
   a false literal.
4. Run `make paper`, then run the full required `make test clippy` verification.
   Confirm formatting, a clean tracked worktree, and that the temporary plan is
   removed before the final push.

