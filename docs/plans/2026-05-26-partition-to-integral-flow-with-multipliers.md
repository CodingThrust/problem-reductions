# Issue 363 Plan: Partition -> IntegralFlowWithMultipliers

Issue: [#363](https://github.com/CodingThrust/problem-reductions/issues/363)  
Title: `[Rule] PARTITION to INTEGRAL FLOW WITH MULTIPLIERS`

## Objective

Implement the witness-preserving reduction `Partition -> IntegralFlowWithMultipliers` using Sahni's 1974 multiplier-flow gadget with the relay bottleneck fix documented in the issue body and comments. The rule must map:

- even-total `Partition` instances to a relay network whose sink inflow is forced to equal `S / 2`, and
- odd-total instances to a fixed infeasible `IntegralFlowWithMultipliers` instance.

Verification mode: default. No `--no-verify`.

## Reference Notes

- Primary source: Sartaj Sahni, *Computationally Related Problems*, SIAM J. Comput. 3(4):262-279, 1974, Section 2.2 / Fig. 2.2.1 (`sum of subsets -> N(i)`).
- Catalog source: Garey and Johnson, ND33, `Integral Flow With Multipliers`.
- Issue comments already resolved the earlier false-positive construction by adding relay vertex `w` and bottleneck arc `(w, t)` with capacity `S / 2`.

## Action Pipeline

This plan follows `.claude/skills/add-rule/SKILL.md` Steps 1-7.

## Batch 1: Steps 1-5.5 (verification, implementation, tests, example-db)

### 1. Mathematical verification

- Re-state the reduction precisely in repository semantics:
  - Source: `Partition`
  - Target: `IntegralFlowWithMultipliers`
  - Witness on source: binary subset vector over `sizes`
  - Witness on target: integral arc-flow vector in graph arc order
- Verify the two branches:
  - odd total `S`: fixed 3-vertex NO instance with `h(u) = 2`, capacities `(1, 1)`, `R = 1`
  - even total `S`: vertices `s, v_1, ..., v_n, w, t`; arcs `(s, v_i)`, `(v_i, w)`, `(w, t)`; multipliers `h(v_i) = a_i`, `h(w) = 1`; requirement `R = S / 2`
- Lock the extraction rule:
  - read the `n` source-item arcs `(s, v_i)`
  - map `flow = 1` to selected item and `flow = 0` to unselected item
  - odd branch returns the all-zero source config

### 2. Implement the reduction

- Add `src/rules/partition_integralflowwithmultipliers.rs`.
- Implement `ReductionPartitionToIntegralFlowWithMultipliers` with:
  - `target: IntegralFlowWithMultipliers`
  - `source_n: usize`
  - `item_arc_count: usize` so extraction can distinguish even vs odd branch reliably
- Implement `ReduceTo<IntegralFlowWithMultipliers> for Partition`.
- Construction details:
  - even branch:
    - vertex numbering `0 = s`, `1..=n = v_i`, `n + 1 = w`, `n + 2 = t`
    - arcs in deterministic order: all `(s, v_i)`, then all `(v_i, w)`, then `(w, t)`
    - capacities: `1`, then `a_i`, then `S / 2`
    - multipliers: source/sink placeholders `1`, item multipliers `a_i`, relay multiplier `1`
    - requirement `S / 2`
  - odd branch:
    - fixed graph `s -> u -> t`
    - capacities `[1, 1]`
    - multipliers `[1, 2, 1]`
    - requirement `1`
- Add exact overhead metadata:
  - `num_vertices = "num_elements + 3"`
  - `num_arcs = "2 * num_elements + 1"`
  - `max_capacity = "total_sum"`
  - `requirement = "total_sum"`
  Notes:
  - these are valid asymptotic upper bounds across both branches; the odd branch is constant size
  - `total_sum` safely upper-bounds both `S / 2` and `max_i a_i`

### 3. Register in `src/rules/mod.rs`

- Add `mod partition_integralflowwithmultipliers;`.

### 4. Write unit tests

- Add `src/unit_tests/rules/partition_integralflowwithmultipliers.rs`.
- Required coverage:
  - closed-loop YES instance using brute force on target and round-tripping to source
  - even-sum NO instance `{3, 5}` proving the bottleneck removes the earlier false positive
  - odd-sum NO instance `{1, 2}` proving the fixed NO target is infeasible
  - structure test on the worked example `{2, 3, 4, 5, 6, 4}`:
    - vertex count `9`
    - arc order and capacities
    - multipliers
    - requirement `12`
  - extraction test from a hand-written feasible target flow
- Reuse `assert_satisfaction_round_trip_from_satisfaction_target` if it fits the witness-preserving pattern.

### 5. Add canonical example to `example_db`

- Add a builder in `src/example_db/rule_builders.rs` with id `partition_to_integralflowwithmultipliers`.
- Use the issue's tutorial instance `A = {2, 3, 4, 5, 6, 4}` and the canonical half-sum witness selecting `{2, 4, 6}`.
- Ensure the target witness matches the rule's deterministic arc order:
  - source arcs: `[1, 0, 1, 0, 1, 0]`
  - relay arcs: `[2, 0, 4, 0, 6, 0]`
  - bottleneck arc: `[12]`

### 5.5. Local rule-level verification before paper

- Run focused tests for:
  - model feasibility expectations
  - new rule unit tests
  - example-db lookup if needed
- Fix any witness-ordering or feasibility mismatches before touching paper.

## Batch 2: Step 6 and Step 7 (paper, exports, fixtures, final verification)

### 6. Document in paper

- Add a `reduction-rule("Partition", "IntegralFlowWithMultipliers", ...)` entry to `docs/paper/reductions.typ`.
- Include:
  - construction summary citing Sahni 1974 / Garey-Johnson ND33
  - correctness proof with the exact-equality argument:
    - cap `(w, t) <= S / 2`
    - sink requirement `>= S / 2`
    - therefore sink inflow `= S / 2`
  - solution extraction from the unit-capacity source-item arcs
  - explicit odd-total preprocessing branch
- Add a worked example derived from the canonical example fixture, starting with `pred-commands()`.

### 7. Regenerate exports and verify

- Run the required generators after the rule and paper are in place:
  - `cargo run --example export_graph`
  - `cargo run --example export_schemas`
  - `make regenerate-fixtures`
- Run default verification commands without `--no-verify`:
  - at minimum `make test clippy`
  - `make paper`
- Inspect `git status --short` and stage only intended tracked outputs. Ignore generated `docs/src/reductions/` exports if they appear untracked/ignored.

## Expected Deliverables

- New reduction source file and tests
- Rule registration
- Canonical example-db entry and regenerated fixture data
- Paper entry for `Partition -> IntegralFlowWithMultipliers`
- Updated reduction graph / schema exports
- Clean implementation commit(s), plan-file removal commit, PR comment, and pushed branch
