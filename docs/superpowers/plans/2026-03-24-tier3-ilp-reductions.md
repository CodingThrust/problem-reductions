# Tier 3 ILP Reductions Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Connect 39 orphan Tier 3 problems to ILP via direct reductions in one PR, with shared linearization helpers.

**Architecture:** Each reduction follows the established pattern: `#[reduction(overhead)]` macro on `impl ReduceTo<ILP<bool/i32>>`, a `ReductionResult` struct with `extract_solution`, and a closed-loop test. A new `ilp_helpers.rs` module provides shared linearization primitives (McCormick, MTZ, flow conservation, big-M, abs-diff, minimax, one-hot decode). Paper entries are already written in `docs/paper/reductions.typ`.

**Tech Stack:** Rust, `#[reduction]` proc macro, `ILP<bool>` / `ILP<i32>` target types, `LinearConstraint` API.

**Spec:** `docs/superpowers/specs/2026-03-24-tier3-ilp-reductions-design.md`
**Paper entries:** `docs/paper/reductions.typ` (search for each `#reduction-rule("<ProblemName>", "ILP")`)

**Status:** Paper entries are committed and reviewed. All 39 entries have standardized multiline ILP equation blocks + detailed prose constructions. 9 complex entries have been expanded with full variable indexing, big-M values, and flow schemes. All symbols verified against problem definitions.

---

## CRITICAL: Paper Is Ground Truth

**The Typst paper (`docs/paper/reductions.typ`) is the authoritative source for every ILP formulation.** Each reduction-rule entry contains a standardized multiline equation block showing the complete ILP (objective/find + constraints + domain), plus prose explaining variable meanings and solution extraction. These entries have been reviewed and verified against the model files.

**When implementing each reduction in Rust, you MUST:**
1. **Read the paper entry first** — find the `#reduction-rule("<ProblemName>", "ILP")` block
2. **Implement exactly the formulation described in the paper** — same variables, same constraints, same extraction logic. Do NOT invent a different formulation.
3. **Cross-check** — if you find the paper's formulation seems wrong or incomplete, STOP and flag it for human review. Do not silently deviate.
4. **The spec file is secondary** — it provides metadata (ILP type, helpers, dims) but the paper has the precise mathematical construction. When they conflict, the paper wins.

---

## File Structure

**New files (40 total):**
- `src/rules/ilp_helpers.rs` — shared helper module
- `src/unit_tests/rules/ilp_helpers.rs` — helper tests
- 39 rule files: `src/rules/<source>_ilp.rs`
- 39 test files: `src/unit_tests/rules/<source>_ilp.rs`

**Modified files:**
- `src/rules/mod.rs` — 39 module declarations + 39 canonical_rule_example_specs calls

---

## Reference Files

Before implementing ANY task, read these files to understand the patterns:

- **Rule template:** `src/rules/maximalis_ilp.rs` (complete ILP reduction example)
- **Test template:** `src/unit_tests/rules/knapsack_ilp.rs` (closed-loop test pattern)
- **Test helpers:** `src/rules/test_helpers.rs` (assertion functions)
- **ILP model:** `src/models/algebraic/ilp.rs` (LinearConstraint, ILP struct, ObjectiveSense)
- **Paper formulations:** `docs/paper/reductions.typ` lines 8206-8607 (mathematical reference for each reduction)

---

## Task 0: Helper Module

**Files:**
- Create: `src/rules/ilp_helpers.rs`
- Create: `src/unit_tests/rules/ilp_helpers.rs`
- Modify: `src/rules/mod.rs` (add module declaration)

- [ ] **Step 0.1: Add module declaration to mod.rs**

Add inside the `#[cfg(feature = "ilp-solver")]` block in `src/rules/mod.rs`:
```rust
#[cfg(feature = "ilp-solver")]
pub(crate) mod ilp_helpers;
```

- [ ] **Step 0.2: Write helper tests (TDD)**

Create `src/unit_tests/rules/ilp_helpers.rs` with tests for all 7 helpers:
```rust
// Test mccormick_product: verify 3 constraints y<=x_a, y<=x_b, y>=x_a+x_b-1
// Test mtz_ordering: verify arc constraints + bound constraints
// Test flow_conservation: verify demand equations at each node
// Test big_m_activation: verify f <= M*y
// Test abs_diff_le: verify two constraints for |a-b| <= z
// Test minimax_constraints: verify z >= expr_i for each expr
// Test one_hot_decode: verify correct index extraction
```

- [ ] **Step 0.3: Implement ilp_helpers.rs**

Create `src/rules/ilp_helpers.rs` with 7 public functions matching the spec's Phase 0 signatures. Reference `src/models/algebraic/ilp.rs` for `LinearConstraint` API.

- [ ] **Step 0.4: Run tests, verify pass**

```bash
cargo test --features ilp-solver ilp_helpers -- --nocapture
```

- [ ] **Step 0.5: Commit**

```bash
git add src/rules/ilp_helpers.rs src/unit_tests/rules/ilp_helpers.rs src/rules/mod.rs
git commit -m "feat: add shared ILP linearization helpers (McCormick, MTZ, flow, big-M, abs-diff, minimax, one-hot)"
```

---

## Task 1: Flow-based reductions (9 rules)

**For each rule below, follow this sub-pattern:**
1. **Read the paper entry FIRST** (`docs/paper/reductions.typ`) — this is the ground truth for the ILP formulation (variables, constraints, objective, extraction). Implement exactly what it says.
2. Read the model file (`src/models/<path>/<model>.rs`) — note `dims()`, `Value`, getters for overhead expressions
3. Write the test file (`src/unit_tests/rules/<source>_ilp.rs`) — closed-loop test with small instance
4. Write the rule file (`src/rules/<source>_ilp.rs`) — implement the paper's formulation in Rust, with extract_solution + canonical example
5. Add module + specs registration to `src/rules/mod.rs`
6. Run `cargo test --features ilp-solver <source>_ilp`
7. Run `cargo clippy --features ilp-solver`

### Task 1.1: IntegralFlowHomologousArcs → ILP

**Files:**
- Create: `src/rules/integralflowhomologousarcs_ilp.rs`
- Create: `src/unit_tests/rules/integralflowhomologousarcs_ilp.rs`
- Modify: `src/rules/mod.rs`
- Model: `src/models/graph/integral_flow_homologous_arcs.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8209

**ILP type:** `ILP<i32>`. **Value:** `Or`. **Extract:** Direct (f_a values).
**Formulation:** Integer f_a per arc. Capacity, conservation, homologous equality, requirement.
**Helpers:** `flow_conservation`

- [ ] **Step 1.1.1:** Write test — construct small network (4-5 nodes, 6-8 arcs, 1-2 homologous pairs), test closed-loop with `assert_satisfaction_round_trip_from_satisfaction_target`
- [ ] **Step 1.1.2:** Write rule — `impl ReduceTo<ILP<i32>>`, overhead = `{ num_vars = "num_arcs", num_constraints = "num_arcs + num_vertices + num_homologous_pairs" }`
- [ ] **Step 1.1.3:** Register in mod.rs, run tests + clippy

### Task 1.2: IntegralFlowWithMultipliers → ILP

**Files:** `src/rules/integralflowwithmultipliers_ilp.rs` + test
- Model: `src/models/graph/integral_flow_with_multipliers.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8219

**ILP type:** `ILP<i32>`. **Value:** `Or`. **Extract:** Direct.
**Formulation:** Integer f_a per arc. Capacity, multiplier-scaled conservation, requirement.
**Helpers:** `flow_conservation` (adapted for multipliers)

- [ ] **Step 1.2.1-1.2.3:** Test → Rule → Register (same sub-pattern)

### Task 1.3: PathConstrainedNetworkFlow → ILP

**Files:** `src/rules/pathconstrainednetworkflow_ilp.rs` + test
- Model: `src/models/graph/path_constrained_network_flow.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8229

**ILP type:** `ILP<i32>`. **Value:** `Or`. **Extract:** Direct (f_p per path).
**Formulation:** Integer f_p per allowed path. Arc capacity aggregation, requirement.
**Helpers:** None

- [ ] **Step 1.3.1-1.3.3:** Test → Rule → Register

### Task 1.4: DisjointConnectingPaths → ILP

**Files:** `src/rules/disjointconnectingpaths_ilp.rs` + test
- Model: `src/models/graph/disjoint_connecting_paths.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8239

**ILP type:** `bool`. **Value:** `Or`. **Extract:** OR over commodities → binary edge selection.
**Formulation:** Binary f^k_{uv} per commodity per arc. Conservation, vertex-disjointness (Σ_k ≤ 1), order vars for subtour elimination.
**Helpers:** `flow_conservation`

- [ ] **Step 1.4.1-1.4.3:** Test → Rule → Register

### Task 1.5: LengthBoundedDisjointPaths → ILP

**Files:** `src/rules/lengthboundeddisjointpaths_ilp.rs` + test
- Model: `src/models/graph/length_bounded_disjoint_paths.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8249

**ILP type:** `ILP<i32>`. **Value:** `Or`. **Extract:** Flow vars → vertex indicators per path slot.
**Formulation:** Binary flow + integer hop counters per commodity. Conservation, disjointness, hop ≤ L.
**Helpers:** `flow_conservation`

- [ ] **Step 1.5.1-1.5.3:** Test → Rule → Register

### Task 1.6: MixedChinesePostman → ILP

**Files:** `src/rules/mixedchinesepostman_ilp.rs` + test
- Model: `src/models/graph/mixed_chinese_postman.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8259

**ILP type:** `ILP<i32>`. **Value:** `Or`. **Extract:** Orientation bits d_e.
**Formulation:** Binary orientation + integer augmentation + connectivity flow. Euler balance, length bound.
**Helpers:** `flow_conservation`, `big_m_activation`

- [ ] **Step 1.6.1-1.6.3:** Test → Rule → Register

### Task 1.7: RuralPostman → ILP

**Files:** `src/rules/ruralpostman_ilp.rs` + test
- Model: `src/models/graph/rural_postman.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8269

**ILP type:** `ILP<i32>`. **Value:** `Or`. **Extract:** Direct (t_e ternary multiplicity, `dims() = vec![3; num_edges]`).
**Formulation:** Integer t_e ∈ {0,1,2} + binary y_e + flow. Required coverage, even degree, connectivity, length bound.
**Helpers:** `flow_conservation`, `big_m_activation`

- [ ] **Step 1.7.1-1.7.3:** Test → Rule → Register

### Task 1.8: StackerCrane → ILP

**Files:** `src/rules/stackercrane_ilp.rs` + test
- Model: `src/models/misc/stacker_crane.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8279

**ILP type:** `ILP<i32>`. **Value:** `Or`. **Extract:** One-hot decode → arc permutation (`dims() = vec![m; m]`).
**Formulation:** Binary x_{a,p} position-assignment + McCormick z for consecutive pairs. Precomputed shortest-path connector costs.
**Helpers:** `mccormick_product`, `one_hot_decode`

- [ ] **Step 1.8.1-1.8.3:** Test → Rule → Register

### Task 1.9: SteinerTreeInGraphs → ILP

**Files:** `src/rules/steinertreeingraphs_ilp.rs` + test
- Model: `src/models/graph/steiner_tree_in_graphs.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8289

**ILP type:** `ILP<bool>`. **Value:** `Min<W::Sum>` (optimization). **Extract:** Direct (edge selection).
**Formulation:** Binary y_e + multi-commodity flow. Same pattern as existing SteinerTree→ILP.
**Helpers:** `flow_conservation`, `big_m_activation`

- [ ] **Step 1.9.1-1.9.3:** Test (use `assert_optimization_round_trip_from_optimization_target`) → Rule → Register

- [ ] **Step 1.10: Run full flow-based test suite + commit**

```bash
cargo test --features ilp-solver -- integralflow steiner disjoint lengthbounded mixed rural stacker
cargo clippy --features ilp-solver
git add src/rules/*_ilp.rs src/unit_tests/rules/*_ilp.rs src/rules/mod.rs
git commit -m "feat: add 9 flow-based Tier 3 ILP reductions"
```

---

## Task 2: Scheduling reductions (7 rules)

**Common note:** FlowShopScheduling, MinimumTardinessSequencing, SequencingToMinimizeWeightedTardiness use Lehmer-code configs. Extract via: sort jobs by ILP completion times → derive permutation → convert to Lehmer code. Use a shared `permutation_to_lehmer()` helper (can be added to `ilp_helpers.rs`).

### Task 2.1: FlowShopScheduling → ILP
- Model: `src/models/misc/flow_shop_scheduling.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8301
- **ILP type:** `ILP<i32>`. **Value:** `Or`. **Extract:** Completion times → sort → Lehmer code.
- [ ] **Step 2.1.1-2.1.3:** Test → Rule → Register

### Task 2.2: MinimumTardinessSequencing → ILP
- Model: `src/models/misc/minimum_tardiness_sequencing.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8311
- **ILP type:** `ILP<i32>`. **Value:** `Min<usize>` (optimization). **Extract:** Position decode → Lehmer code.
- [ ] **Step 2.2.1-2.2.3:** Test (optimization round-trip) → Rule → Register

### Task 2.3: ResourceConstrainedScheduling → ILP
- Model: `src/models/misc/resource_constrained_scheduling.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8321
- **ILP type:** `ILP<bool>`. **Value:** `Or`. **Extract:** Time-slot decode.
- [ ] **Step 2.3.1-2.3.3:** Test → Rule → Register

### Task 2.4: SequencingToMinimizeMaximumCumulativeCost → ILP
- Model: `src/models/misc/sequencing_to_minimize_maximum_cumulative_cost.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8331
- **ILP type:** `ILP<i32>`. **Value:** `Or`. **Extract:** Position decode → Lehmer code.
- [ ] **Step 2.4.1-2.4.3:** Test → Rule → Register

### Task 2.5: SequencingToMinimizeWeightedTardiness → ILP
- Model: `src/models/misc/sequencing_to_minimize_weighted_tardiness.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8341
- **ILP type:** `ILP<i32>`. **Value:** `Or`. **Extract:** Completion times → sort → Lehmer code.
- [ ] **Step 2.5.1-2.5.3:** Test → Rule → Register

### Task 2.6: SequencingWithReleaseTimesAndDeadlines → ILP
- Model: `src/models/misc/sequencing_with_release_times_and_deadlines.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8351
- **ILP type:** `ILP<bool>`. **Value:** `Or`. **Extract:** Start-time decode → sort → Lehmer code.
- [ ] **Step 2.6.1-2.6.3:** Test → Rule → Register

### Task 2.7: TimetableDesign → ILP
- Model: `src/models/misc/timetable_design.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8361
- **ILP type:** `ILP<bool>`. **Value:** `Or`. **Extract:** Direct (binary tensor).
- [ ] **Step 2.7.1-2.7.3:** Test → Rule → Register

- [ ] **Step 2.8: Run full scheduling test suite + commit**

```bash
cargo test --features ilp-solver -- flowshop tardiness resourceconstrained sequencing timetable
cargo clippy --features ilp-solver
git commit -m "feat: add 7 scheduling Tier 3 ILP reductions"
```

---

## Task 3: Position/Assignment + McCormick reductions (6 rules)

### Task 3.1: HamiltonianPath → ILP
- Model: `src/models/graph/hamiltonian_path.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8373
- **ILP type:** `ILP<bool>`. **Value:** `Or`. **Extract:** One-hot decode → vertex permutation (`dims() = vec![n; n]`).
- **Helpers:** `mccormick_product`, `one_hot_decode`
- [ ] **Step 3.1.1-3.1.3:** Test → Rule → Register

### Task 3.2: BottleneckTravelingSalesman → ILP
- Model: `src/models/graph/bottleneck_traveling_salesman.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8383
- **ILP type:** `ILP<i32>`. **Value:** `Min<i32>` (optimization). **Extract:** Position tour → edge selection (`dims() = vec![2; num_edges]`).
- **Helpers:** `mccormick_product`, `minimax_constraints`, `one_hot_decode`
- [ ] **Step 3.2.1-3.2.3:** Test (optimization round-trip) → Rule → Register

### Task 3.3: LongestCircuit → ILP
- Model: `src/models/graph/longest_circuit.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8393
- **ILP type:** `ILP<bool>`. **Value:** `Or`. **Extract:** Direct (binary edge selection).
- **Formulation:** Degree-2 vertex selection + flow connectivity (NOT position-assignment). No McCormick.
- **Helpers:** `flow_conservation`
- [ ] **Step 3.3.1-3.3.3:** Test → Rule → Register

### Task 3.4: QuadraticAssignment → ILP
- Model: `src/models/algebraic/quadratic_assignment.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8403
- **ILP type:** `ILP<bool>`. **Value:** `Min<i64>` (optimization). **Extract:** One-hot decode → injection (`dims() = vec![num_locations; num_facilities]`).
- **Helpers:** `mccormick_product`, `one_hot_decode`
- [ ] **Step 3.4.1-3.4.3:** Test (optimization round-trip) → Rule → Register

### Task 3.5: OptimalLinearArrangement → ILP
- Model: `src/models/graph/optimal_linear_arrangement.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8413
- **ILP type:** `ILP<i32>`. **Value:** `Or`. **Extract:** One-hot decode → vertex positions (`dims() = vec![n; n]`).
- **Helpers:** `abs_diff_le`, `one_hot_decode`
- [ ] **Step 3.5.1-3.5.3:** Test → Rule → Register

### Task 3.6: SubgraphIsomorphism → ILP
- Model: `src/models/graph/subgraph_isomorphism.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8423
- **ILP type:** `ILP<bool>`. **Value:** `Or`. **Extract:** One-hot decode → injection (`dims() = vec![n_host; n_pattern]`).
- **Formulation:** No McCormick — direct non-edge constraints `x_{v,u} + x_{w,u'} ≤ 1`.
- [ ] **Step 3.6.1-3.6.3:** Test → Rule → Register

- [ ] **Step 3.7: Run full position/assignment test suite + commit**

```bash
cargo test --features ilp-solver -- hamiltonianpath bottleneck longestcircuit quadratic optimal subgraph
cargo clippy --features ilp-solver
git commit -m "feat: add 6 position/assignment Tier 3 ILP reductions"
```

---

## Task 4: Graph structure reductions (7 rules)

### Task 4.1: AcyclicPartition → ILP
- Model: `src/models/graph/acyclic_partition.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8435
- **ILP type:** `ILP<i32>`. **Value:** `Or`. **Extract:** One-hot decode x_{v,c} → partition label (`dims() = vec![n; n]`).
- **Formulation:** Binary assignment + McCormick same-class indicators + class ordering for quotient DAG.
- **Helpers:** `mccormick_product`, `one_hot_decode`
- [ ] **Step 4.1.1-4.1.3:** Test → Rule → Register

### Task 4.2: BalancedCompleteBipartiteSubgraph → ILP
- Model: `src/models/graph/balanced_complete_bipartite_subgraph.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8445
- **ILP type:** `ILP<bool>`. **Value:** `Or`. **Extract:** Direct (binary selection).
- **Formulation:** Binary x_l, y_r. Balance + non-edge constraints. No McCormick.
- [ ] **Step 4.2.1-4.2.3:** Test → Rule → Register

### Task 4.3: BicliqueCover → ILP
- Model: `src/models/graph/biclique_cover.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8455
- **ILP type:** `ILP<bool>`. **Value:** `Min<i32>` (optimization). **Extract:** Direct (membership bits).
- **Helpers:** `mccormick_product`
- [ ] **Step 4.3.1-4.3.3:** Test (optimization round-trip) → Rule → Register

### Task 4.4: BiconnectivityAugmentation → ILP
- Model: `src/models/graph/biconnectivity_augmentation.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8465
- **ILP type:** `ILP<i32>`. **Value:** `Or`. **Extract:** Direct (binary edge selection).
- **Formulation:** Binary y_e + flow for 2-vertex-connectivity (per-vertex-deletion connectivity check).
- **Helpers:** `flow_conservation`, `big_m_activation`
- [ ] **Step 4.4.1-4.4.3:** Test → Rule → Register

### Task 4.5: BoundedComponentSpanningForest → ILP
- Model: `src/models/graph/bounded_component_spanning_forest.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8475
- **ILP type:** `ILP<i32>`. **Value:** `Or`. **Extract:** Component label decode.
- **Formulation:** Binary x_{v,c} assignment + weight bounds + flow connectivity within components.
- **Helpers:** `flow_conservation`, `one_hot_decode`
- [ ] **Step 4.5.1-4.5.3:** Test → Rule → Register

### Task 4.6: MinimumCutIntoBoundedSets → ILP
- Model: `src/models/graph/minimum_cut_into_bounded_sets.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8485
- **ILP type:** `ILP<bool>`. **Value:** `Or`. **Extract:** Direct (partition bit-vector).
- **Formulation:** Binary x_v + binary y_e. Balance bounds + cut linking.
- [ ] **Step 4.6.1-4.6.3:** Test → Rule → Register

### Task 4.7: StrongConnectivityAugmentation → ILP
- Model: `src/models/graph/strong_connectivity_augmentation.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8495
- **ILP type:** `ILP<i32>`. **Value:** `Or`. **Extract:** Direct (binary arc selection).
- **Formulation:** Binary y_a + bidirectional multi-commodity flow from root.
- **Helpers:** `flow_conservation`, `big_m_activation`
- [ ] **Step 4.7.1-4.7.3:** Test → Rule → Register

- [ ] **Step 4.8: Run full graph structure test suite + commit**

```bash
cargo test --features ilp-solver -- acyclicpartition balanced biclique biconnectivity bounded minimumcut strongconnectivity
cargo clippy --features ilp-solver
git commit -m "feat: add 7 graph structure Tier 3 ILP reductions"
```

---

## Task 5: Matrix/encoding reductions (5 rules)

### Task 5.1: BMF → ILP
- Model: `src/models/algebraic/bmf.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8507
- **ILP type:** `ILP<bool>`. **Value:** `Min<i32>` (optimization). **Extract:** Direct (factor matrix bits).
- **Formulation:** McCormick for Boolean products, OR-of-ANDs reconstruction, Hamming distance.
- **Helpers:** `mccormick_product`
- [ ] **Step 5.1.1-5.1.3:** Test (optimization round-trip) → Rule → Register

### Task 5.2: ConsecutiveBlockMinimization → ILP
- Model: `src/models/algebraic/consecutive_block_minimization.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8517
- **ILP type:** `ILP<bool>`. **Value:** `Or`. **Extract:** One-hot decode → column permutation (`dims() = vec![num_cols; num_cols]`).
- **Helpers:** `one_hot_decode`
- [ ] **Step 5.2.1-5.2.3:** Test → Rule → Register

### Task 5.3: ConsecutiveOnesMatrixAugmentation → ILP
- Model: `src/models/algebraic/consecutive_ones_matrix_augmentation.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8527
- **ILP type:** `ILP<bool>`. **Value:** `Or`. **Extract:** One-hot decode → column permutation.
- [ ] **Step 5.3.1-5.3.3:** Test → Rule → Register

### Task 5.4: ConsecutiveOnesSubmatrix → ILP
- Model: `src/models/algebraic/consecutive_ones_submatrix.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8537
- **ILP type:** `ILP<bool>`. **Value:** `Or`. **Extract:** Direct (s_j selection bits, `dims() = vec![2; num_cols]`).
- **Formulation:** Binary s_j + auxiliary permutation x_{c,p} + C1P interval constraints.
- [ ] **Step 5.4.1-5.4.3:** Test → Rule → Register

### Task 5.5: SparseMatrixCompression → ILP
- Model: `src/models/algebraic/sparse_matrix_compression.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8547
- **ILP type:** `ILP<bool>`. **Value:** `Or`. **Extract:** One-hot decode → shift assignment.
- [ ] **Step 5.5.1-5.5.3:** Test → Rule → Register

- [ ] **Step 5.6: Run full matrix/encoding test suite + commit**

```bash
cargo test --features ilp-solver -- bmf consecutiveblock consecutiveones sparse
cargo clippy --features ilp-solver
git commit -m "feat: add 5 matrix/encoding Tier 3 ILP reductions"
```

---

## Task 6: Sequence/misc reductions (5 rules)

### Task 6.1: ShortestCommonSupersequence → ILP
- Model: `src/models/misc/shortest_common_supersequence.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8559
- **ILP type:** `ILP<bool>`. **Value:** `Or`. **Extract:** Symbol sequence extraction.
- [ ] **Step 6.1.1-6.1.3:** Test → Rule → Register

### Task 6.2: StringToStringCorrection → ILP
- Model: `src/models/misc/string_to_string_correction.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8569
- **ILP type:** `ILP<bool>`. **Value:** `Or`. **Extract:** Operation indicator → scalar operation code.
- [ ] **Step 6.2.1-6.2.3:** Test → Rule → Register

### Task 6.3: PaintShop → ILP
- Model: `src/models/misc/paintshop.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8579
- **ILP type:** `ILP<bool>`. **Value:** `Min<i32>` (optimization). **Extract:** Direct (x_i first-occurrence color bits, `dims() = vec![2; num_cars]`).
- [ ] **Step 6.3.1-6.3.3:** Test (optimization round-trip) → Rule → Register

### Task 6.4: IsomorphicSpanningTree → ILP
- Model: `src/models/graph/isomorphic_spanning_tree.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8589
- **ILP type:** `ILP<bool>`. **Value:** `Or`. **Extract:** One-hot decode → bijection (`dims() = vec![n; n]`).
- **Formulation:** Pure bijection x_{u,v} with non-edge constraints (no flow needed).
- [ ] **Step 6.4.1-6.4.3:** Test → Rule → Register

### Task 6.5: RootedTreeStorageAssignment → ILP
- Model: `src/models/set/rooted_tree_storage_assignment.rs`
- Paper: search for `#reduction-rule("ProblemName", "ILP")` ~line 8599
- **ILP type:** `ILP<i32>`. **Value:** `Or`. **Extract:** One-hot parent decode → parent array (`dims() = vec![n; n]`).
- **Formulation:** Binary p_{v,u} parent indicators + integer depths + subset path extension costs.
- [ ] **Step 6.5.1-6.5.3:** Test → Rule → Register

- [ ] **Step 6.6: Run full sequence/misc test suite + commit**

```bash
cargo test --features ilp-solver -- shortestcommon stringtostring paintshop isomorphicspanning rootedtreestorage
cargo clippy --features ilp-solver
git commit -m "feat: add 5 sequence/misc Tier 3 ILP reductions"
```

---

## Task 7: Final verification and PR

- [ ] **Step 7.1: Full test suite**

```bash
make check
cargo test --features ilp-solver
```

- [ ] **Step 7.2: Paper completeness check**

```bash
make paper
```
Paper entries are already committed. Verify no new completeness warnings after Rust reductions are registered (the `#[reduction]` macro registrations should match the paper's `reduction-rule` entries).

- [ ] **Step 7.3: Coverage check**

```bash
make coverage
```
Verify >95% coverage on new code.

- [ ] **Step 7.4: Final commit and PR**

```bash
git add -A
git commit -m "feat: add 39 Tier 3 ILP reductions + shared helpers

Connects all remaining orphan NP-hard problems to ILP, enabling
DefaultSolver dispatch. Includes shared ilp_helpers module with
McCormick, MTZ, flow conservation, big-M, abs-diff, and minimax
linearization primitives.

Closes #728, closes #733.
Ref #762."
```

Create PR targeting `main`.

- [ ] **Step 7.5: Post-merge cleanup**

- Update #762 body: move 39 problems from Tier 3 to Tier 1
- Close #728 (TimetableDesign→ILP) and #733 (IntegralFlowHomologousArcs→ILP)
- File separate issues for deferred: PartialFeedbackEdgeSet→ILP, RootedTreeArrangement→ILP
