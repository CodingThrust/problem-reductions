# Tier 3 ILP Reductions — Design Spec

**Date:** 2026-03-24
**Scope:** One PR adding 39 `→ ILP` reductions for Tier 3 orphan problems, plus a shared helper module.
**Deferred:** PartialFeedbackEdgeSet (no polynomial-size correct ILP for L < n), RootedTreeArrangement (compound `vec![n; 2*n]` config too complex for batch).
**Tracking issue:** #762 (DefaultSolver classification)

---

## Goal

Connect 39 of 41 isolated Tier 3 problem types to the reduction graph via direct ILP reductions. Two problems (PartialFeedbackEdgeSet, RootedTreeArrangement) are deferred to separate issues due to formulation complexity.

## Deliverables

1. `src/rules/ilp_helpers.rs` — shared linearization helpers (with unit tests)
2. 39 new reduction files `src/rules/<problem>_ilp.rs` (feature-gated under `#[cfg(feature = "ilp-solver")]`)
3. 39 entries in `src/rules/mod.rs`: module declarations + `canonical_rule_example_specs()` aggregation
4. 39 closed-loop tests in corresponding unit test files
5. 39 `reduction-rule` entries in `docs/paper/reductions.typ`
6. Updated #762 body (move Tier 3 → Tier 1)

---

## Problem Classification

### Value types (optimization vs satisfaction)

**Optimization** (`Min`/`Max` — use `assert_optimization_round_trip_from_optimization_target`):
- BottleneckTravelingSalesman (`Min<i32>`), MinimumTardinessSequencing (`Min<usize>`),
  QuadraticAssignment (`Min<i64>`), BMF (`Min<i32>`), PaintShop (`Min<i32>`),
  SteinerTreeInGraphs (`Min<W::Sum>`)

**Satisfaction** (`Or` — use `assert_satisfaction_round_trip` or satisfaction variant):
- All other 33 problems

### Config-space encodings requiring non-trivial `extract_solution`

| Encoding | Problems | Extraction strategy |
|----------|----------|-------------------|
| **Lehmer code** `[n, n-1, ..., 1]` | FlowShopScheduling, MinimumTardinessSequencing, SequencingToMinimizeWeightedTardiness | Sort jobs by ILP completion times → derive permutation → convert to Lehmer code |
| **Vertex permutation** `vec![n; n]` | HamiltonianPath, OptimalLinearArrangement, ConsecutiveBlockMinimization, AcyclicPartition | One-hot decode: for each position/vertex, find the 1 in the assignment row |
| **Arc permutation** `vec![m; m]` | StackerCrane | Position-assignment decode: for each position, find the selected arc |
| **Injection** `vec![m; k]` | SubgraphIsomorphism, QuadraticAssignment | One-hot decode per source element |
| **Parent array** `vec![n; n]` | RootedTreeStorageAssignment | Decode parent-selection one-hot matrix → parent index per node |
| **Bijection** `vec![n; n]` | IsomorphicSpanningTree | One-hot decode tree-vertex → graph-vertex |
| **Compound** `vec![n; 2*n]` | *(RootedTreeArrangement — deferred)* | — |
| **Binary** `vec![2; ...]` | All others | Direct identity or first-k prefix extraction |
| **Ternary** `vec![3; num_edges]` | RuralPostman | Integer flow variable → clamp to {0,1,2} per edge |

---

## Phase 0: Helper Module

**File:** `src/rules/ilp_helpers.rs`

Seven helper functions returning `Vec<LinearConstraint>` (or single `LinearConstraint`):

```rust
/// McCormick linearization: y = x_a * x_b (both binary).
/// Returns 3 constraints: y ≤ x_a, y ≤ x_b, y ≥ x_a + x_b - 1.
pub fn mccormick_product(y_idx: usize, x_a: usize, x_b: usize) -> Vec<LinearConstraint>

/// MTZ topological ordering for directed arcs.
/// For each arc (u→v): o_v - o_u ≥ 1 - M*(x_u + x_v).
/// When both x_u=0, x_v=0 (both kept): enforces o_v > o_u.
/// When either x_u=1 or x_v=1 (removed): constraint is slack.
/// Also emits bound constraints: x_i ≤ 1, 0 ≤ o_i ≤ n-1.
/// Matches the pattern in minimumfeedbackvertexset_ilp.rs.
pub fn mtz_ordering(
    arcs: &[(usize, usize)],
    n: usize,
    x_offset: usize,
    o_offset: usize,
) -> Vec<LinearConstraint>

/// Flow conservation at each node.
/// For each node u: Σ_{(u,v)} f_{uv} - Σ_{(v,u)} f_{vu} = demand[u].
pub fn flow_conservation(
    arcs: &[(usize, usize)],
    num_nodes: usize,
    flow_idx: &dyn Fn(usize) -> usize,
    demand: &[f64],
) -> Vec<LinearConstraint>

/// Big-M activation: f ≤ M * y. Single constraint.
pub fn big_m_activation(f_idx: usize, y_idx: usize, big_m: f64) -> LinearConstraint

/// Absolute value linearization: |a - b| ≤ z.
/// Returns 2 constraints: a - b ≤ z, b - a ≤ z.
pub fn abs_diff_le(a_idx: usize, b_idx: usize, z_idx: usize) -> Vec<LinearConstraint>

/// Minimax: z ≥ expr_i for each expression.
/// Each expr is a list of (var_idx, coeff) terms.
pub fn minimax_constraints(z_idx: usize, expr_terms: &[Vec<(usize, f64)>]) -> Vec<LinearConstraint>

/// One-hot to index extraction: given n*k binary assignment vars,
/// decode position p → value v where x_{v,p} = 1.
/// Shared by all permutation/assignment-based reductions.
pub fn one_hot_decode(solution: &[usize], num_items: usize, num_slots: usize, var_offset: usize) -> Vec<usize>
```

The helper module gets its own unit tests verifying constraint correctness.

No new types introduced. Existing Tier 1/2 reductions are **not** refactored — helpers are used only by new Tier 3 code.

---

## Phase 1: Flow-based (9 reductions)

| Problem | Value | ILP type | Variables | Key constraints | Helpers | Extract |
|---------|-------|----------|-----------|-----------------|---------|---------|
| IntegralFlowHomologousArcs | `Or` | `i32` | Integer f_a per arc | Capacity, conservation, homologous equality, requirement | `flow_conservation` | Direct (f_a values) |
| IntegralFlowWithMultipliers | `Or` | `i32` | Integer f_a per arc | Capacity, modified conservation (multiplier factors), requirement | `flow_conservation` | Direct |
| PathConstrainedNetworkFlow | `Or` | `i32` | Integer f_p per allowed path | Capacity aggregation per arc, flow requirement | — | Direct |
| DisjointConnectingPaths | `Or` | `bool` | Binary f^k_{uv} per commodity per arc | Conservation per commodity, vertex-disjointness (Σ_k ≤ 1 at non-terminals) | `flow_conservation` | Reconstruct edge selection from flow variables |
| LengthBoundedDisjointPaths | `Or` | `i32` | Binary f^k_{uv} + integer hop h^k_v per commodity | Conservation, disjointness, hop count h^k_v ≤ L per commodity | `flow_conservation` | Reconstruct edge selection from flow variables |
| MixedChinesePostman | `Or` | `i32` | Integer traversal t_a + binary orientation d_e | Euler balance (in = out), required edge/arc coverage ≥ 1 | `flow_conservation` | Direct (traversal counts) |
| RuralPostman | `Or` | `i32` | Integer t_e ∈ {0,1,2} per edge (traversal multiplicity) | Required edge coverage (t_e ≥ 1), Euler balance (even degree at each vertex), connectivity via flow, total cost ≤ bound | `flow_conservation`, `big_m_activation` | Direct (t_e values map to `dims() = vec![3; num_edges]`) |
| StackerCrane | `Or` | `i32` | Binary x_{a,k} (arc a at position k) + shortest-path cost auxiliaries | Position-assignment (each position gets one required arc, each arc used once), inter-arc connection cost via precomputed shortest paths, total ≤ bound | `big_m_activation` | One-hot decode → arc permutation (`dims() = vec![m; m]`) |
| SteinerTreeInGraphs | `Min<W>` | `bool` | Binary y_e + multi-commodity flow f^t_{uv} | Conservation, capacity linking (same pattern as SteinerTree→ILP); minimize Σ w_e·y_e | `flow_conservation`, `big_m_activation` | Direct (edge selection) |

---

## Phase 2: Scheduling (7 reductions)

All scheduling problems with Lehmer-code configs share a common extraction pattern: ILP ordering variables → sort to get permutation → convert permutation to Lehmer code.

| Problem | Value | ILP type | Variables | Key constraints | Helpers | Extract |
|---------|-------|----------|-----------|-----------------|---------|---------|
| FlowShopScheduling | `Or` | `i32` | Binary y_{ij} (job i before j) + integer C_{jm} (completion on machine m) | Machine precedence: C_{j,m+1} ≥ C_{j,m} + p_{j,m+1}; ordering via big-M; makespan ≤ deadline | `big_m_activation` | Completion times → sort → Lehmer code |
| MinimumTardinessSequencing | `Min<usize>` | `i32` | Binary y_{ij} + integer C_j | Ordering via big-M, precedence constraints; objective: minimize Σ tardy_j (binary indicators for C_j > d_j) | `big_m_activation` | Completion times → sort → Lehmer code |
| ResourceConstrainedScheduling | `Or` | `bool` | Binary x_{jt} (job j starts at time t) | One start per job, precedence, resource capacity per period, deadline | — | Time-indexed decode → Lehmer code |
| SequencingToMinimizeMaximumCumulativeCost | `Or` | `i32` | Binary y_{ij} + integer C_j | Ordering via big-M; cumulative cost ≤ bound (feasibility, not minimax) | `big_m_activation` | Completion times → sort → Lehmer code |
| SequencingToMinimizeWeightedTardiness | `Or` | `i32` | Binary y_{ij} + integer C_j | Ordering via big-M; Σ w_j * max(0, C_j - d_j) ≤ bound (feasibility) | `big_m_activation` | Completion times → sort → Lehmer code |
| SequencingWithReleaseTimesAndDeadlines | `Or` | `bool` | Binary x_{jt} (job j at time t) | Release: no start before r_j, deadline: finish by d_j, non-overlap | — | Time-indexed decode → Lehmer code |
| TimetableDesign | `Or` | `bool` | Binary x_{c,t,h} (craftsman c, task t, period h) | Craftsman exclusivity, task exclusivity, requirement satisfaction | — | Direct (binary) |

---

## Phase 3: Position/Assignment + McCormick (6 reductions)

| Problem | Value | ILP type | Variables | Key constraints | Helpers | Extract |
|---------|-------|----------|-----------|-----------------|---------|---------|
| HamiltonianPath | `Or` | `bool` | Binary x_{v,k} (vertex v at position k) | Row/column assignment, adjacency: McCormick for consecutive pairs | `mccormick_product` | One-hot decode → vertex permutation (`dims() = vec![n; n]`) |
| BottleneckTravelingSalesman | `Min<i32>` | `i32` | Binary x_{v,k} + integer z (bottleneck) | TSP assignment + z ≥ w(u,v) for each used edge (McCormick); minimize z | `mccormick_product`, `minimax_constraints` | Edge selection from assignment matrix (`dims() = vec![2; num_edges]`) |
| LongestCircuit | `Or` | `bool` | Binary y_e (edge selection) + binary s_v (vertex on circuit) + flow vars | Degree: Σ_{e∋v} y_e = 2·s_v; size: Σ y_e ≥ 3; connectivity via root-flow on selected edges; length: Σ w_e·y_e ≥ B | `flow_conservation` | Direct (y_e binary edge vector, `dims() = vec![2; num_edges]`) |
| QuadraticAssignment | `Min<i64>` | `bool` | Binary x_{i,p} (facility i at location p) | Assignment + McCormick for x_{i,p}·x_{j,q}; minimize Σ C_{ij}·D_{f(i),f(j)} | `mccormick_product` | One-hot decode → facility-to-location injection (`dims() = vec![num_locations; num_facilities]`) |
| OptimalLinearArrangement | `Or` | `i32` | Binary x_{v,p} + integer z_{uv} per edge | Assignment + z_{uv} ≥ |π(u)-π(v)| via abs_diff; Σ z_{uv} ≤ bound | `abs_diff_le` | One-hot decode → vertex-to-position (`dims() = vec![n; n]`) |
| SubgraphIsomorphism | `Or` | `bool` | Binary x_{v,u} (pattern v → host u) | Injection (each pattern vertex maps to exactly 1 host vertex, each host vertex used ≤ 1 time) + edge preservation: for each pattern edge (v,w) and host non-edge (u,u'), x_{v,u} + x_{w,u'} ≤ 1 (no McCormick needed) | — | One-hot decode → injection (`dims() = vec![n_host; n_pattern]`) |

---

## Phase 4: Graph structure (7 reductions)

| Problem | Value | ILP type | Variables | Key constraints | Helpers | Extract |
|---------|-------|----------|-----------|-----------------|---------|---------|
| AcyclicPartition | `Or` | `i32` | Binary x_{v,c} (vertex v in class c) + integer o_c (class ordering) + binary s_{uv,c} (same-class indicators per arc per class) | Assignment (Σ_c x_{v,c} = 1); weight bound per class; cost bound on inter-class arcs; same-class: s_{uv,c} via McCormick on x_{u,c}·x_{v,c}; DAG: for each arc (u→v), o_v_class - o_u_class ≥ 1 - M·Σ_c s_{uv,c} | `mccormick_product` | One-hot decode x_{v,c} → partition label (`dims() = vec![n; n]`) |
| BalancedCompleteBipartiteSubgraph | `Or` | `bool` | Binary x_v (side A), y_v (side B) | Balance: Σx = Σy = k; completeness: McCormick for x_u·y_v on non-edges | `mccormick_product` | Direct (binary) |
| BicliqueCover | `Or` | `bool` | Binary z_{v,j} (vertex v in biclique j) | Biclique validity via McCormick, edge coverage | `mccormick_product` | Direct (binary) |
| BiconnectivityAugmentation | `Or` | `i32` | Binary y_e (add edge e) + flow vars for 2-vertex-connectivity | For each vertex v: removing v must leave graph connected. Formulated via flow: for each vertex v and each pair (s,t) of v's neighbors, unit flow from s to t avoiding v, through original + selected edges | `flow_conservation`, `big_m_activation` | Direct (binary edge selection, `dims() = vec![2; num_potential_edges]`) |
| BoundedComponentSpanningForest | `Or` | `i32` | Binary y_e (edge in forest) + integer label l_v (component root ID) + flow vars | Forest structure (no cycles via MTZ on directed version); component assignment via labels; per-component total vertex **weight** ≤ B (not size) | `flow_conservation`, `mtz_ordering` | Edge selection → component label decode (`dims() = vec![2; num_edges]` or label-based) |
| MinimumCutIntoBoundedSets | `Or` | `bool` | Binary x_v (partition side) + binary y_e (cut edge) | Balance: L ≤ Σx_v ≤ U; cut linking: y_e ≥ x_u - x_v and y_e ≥ x_v - x_u; Σ w_e·y_e ≤ bound | — | Direct (binary partition) |
| StrongConnectivityAugmentation | `Or` | `i32` | Binary y_a (add arc) + multi-commodity flow | For each ordered pair (s,t): unit flow from s to t through original + selected arcs | `flow_conservation`, `big_m_activation` | Direct (binary arc selection) |

---

## Phase 5: Matrix/encoding (5 reductions)

| Problem | Value | ILP type | Variables | Key constraints | Helpers | Extract |
|---------|-------|----------|-----------|-----------------|---------|---------|
| BMF | `Min<i32>` | `bool` | Binary a_{ik}, b_{kj} + auxiliary p_{ijk} (McCormick for a_{ik}·b_{kj}) + binary w_{ij} (reconstructed entry) | p_{ijk} via McCormick; w_{ij} ≥ p_{ijk} for all k (OR-of-ANDs); w_{ij} ≤ Σ_k p_{ijk}; minimize Σ |A_{ij} - w_{ij}| | `mccormick_product` | Direct (binary factor matrices) |
| ConsecutiveBlockMinimization | `Or` | `bool` | Binary x_{c,p} (column c at position p) + binary b_{r,p} (block start at row r, position p) | Column permutation (one-hot assignment); block detection: b_{r,p} activated when row r transitions 0→1 at position p; Σ blocks ≤ bound | — | One-hot decode → column permutation (`dims() = vec![num_cols; num_cols]`) |
| ConsecutiveOnesMatrixAugmentation | `Or` | `bool` | Binary x_{c,p} (column permutation) + binary f_{r,j} (flip entry r,j) | Permutation + consecutive-ones property after flips; minimize/bound total flips | — | One-hot decode → column permutation (`dims() = vec![num_cols; num_cols]`) |
| ConsecutiveOnesSubmatrix | `Or` | `bool` | Binary s_j (select column j) + auxiliary binary x_{c,p} (column permutation of selected columns) | Exactly K columns selected (Σ s_j = K); permutation of selected columns; C1P enforced on every row within selected+permuted columns. s_j at indices 0..num_cols (extracted directly). x_{c,p} are auxiliary. | — | Direct (s_j binary selection, `dims() = vec![2; num_cols]`) |
| SparseMatrixCompression | `Or` | `bool` | Binary x_{i,g} (row i in group g) | Row-to-group assignment (one group per row); compatibility: conflicting rows not in same group; num groups ≤ K | — | One-hot decode → group assignment |

---

## Phase 6: Sequence/misc (5 reductions)

| Problem | Value | ILP type | Variables | Key constraints | Helpers | Extract |
|---------|-------|----------|-----------|-----------------|---------|---------|
| ShortestCommonSupersequence | `Or` | `bool` | Binary x_{p,a} (position p has symbol a) + match vars m_{s,j,p} | Symbol assignment + monotone matching for each input string; total length ≤ bound | — | Symbol sequence extraction |
| StringToStringCorrection | `Or` | `bool` | Binary d_{i,j,op} (edit operation at alignment point) | Alignment grid + operation exclusivity + cost ≤ bound | — | Direct (binary operation selection) |
| PaintShop | `Min<i32>` | `bool` | Binary x_i (color for car i's first occurrence) + binary c_p (color-change indicator at position p) | Pairing: second occurrence gets 1-x_i; c_p ≥ color_p - color_{p-1} and c_p ≥ color_{p-1} - color_p; minimize Σ c_p | — | Direct (x_i binary, `dims() = vec![2; num_cars]`) |
| IsomorphicSpanningTree | `Or` | `bool` | Binary x_{u,v} (tree vertex u maps to graph vertex v) | Bijection: one-hot per tree vertex and per graph vertex; edge preservation: for each tree edge {u,w} and graph non-edge {v,z}, x_{u,v} + x_{w,z} ≤ 1 (no McCormick or flow needed — bijection preserving tree edges automatically produces a spanning tree) | — | One-hot decode → bijection (`dims() = vec![n; n]`) |
| RootedTreeStorageAssignment | `Or` | `i32` | Binary p_{v,u} (node v's parent is u) + integer depth d_v | Tree structure: each non-root node has exactly one parent, acyclicity via depth ordering (d_v > d_u if u is parent of v), connectivity; per-subset path cost ≤ bound | — | One-hot parent decode → parent array (`dims() = vec![n; n]`) |

---

## Testing Strategy

- Each reduction gets one `test_<source>_to_ilp_closed_loop` test
- **Optimization problems** (BottleneckTSP, MinTardiness, QAP, BMF, PaintShop): use `assert_optimization_round_trip_from_optimization_target`
- **Satisfaction problems** (all others): use the satisfaction round-trip variant
- Test instances should be small enough for brute-force cross-check (n ≤ 6-8)
- All tests in `src/unit_tests/rules/<problem>_ilp.rs`
- Helper module gets standalone unit tests in `src/unit_tests/rules/ilp_helpers.rs`

## Integration Checklist (per reduction)

Each new reduction file requires:
1. `#[cfg(feature = "ilp-solver")] pub(crate) mod <name>_ilp;` in `src/rules/mod.rs`
2. `specs.extend(<name>_ilp::canonical_rule_example_specs());` in the `#[cfg(feature = "ilp-solver")]` block of `canonical_rule_example_specs()` in `src/rules/mod.rs`
3. `#[reduction(overhead = { ... })]` with verified overhead expressions referencing source-type getter methods
4. Closed-loop test + paper entry

## Paper

Each reduction gets a `reduction-rule` entry in `docs/paper/reductions.typ` with:
- Rule statement describing the formulation
- Proof sketch (variable layout, constraint count, correctness argument)
- Example flag set to `true` where pedagogically useful

## Post-merge

- Update #762 body: move 39 problems from Tier 3 to Tier 1
- Close #728 (TimetableDesign→ILP) and #733 (IntegralFlowHomologousArcs→ILP)
- File separate issues for deferred problems: PartialFeedbackEdgeSet→ILP, RootedTreeArrangement→ILP
