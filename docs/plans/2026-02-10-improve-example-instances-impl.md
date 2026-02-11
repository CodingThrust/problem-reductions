# Improve Example Instances — Implementation Plan (v2)

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace trivial instances (P4, K3, 2-3 variable SAT) in all 30 reduction examples with non-trivial instances (Petersen graph, 5-variable SAT, etc.) to produce better data for the paper.

**Architecture:** Each example file is independent. We swap the instance construction code, update doc comments and print statements, then re-run to regenerate JSON. The code structure (reduce → solve → extract → export) stays identical. We use the existing `petersen()` and `octahedral()` helpers from `src/topology/small_graphs.rs`.

**Tech Stack:** Rust, `problemreductions` crate, `small_graphs` module, BruteForce/ILPSolver

---

## Shared Constants

These concrete instances are referenced across multiple tasks.

### Petersen Graph
```rust
use problemreductions::topology::small_graphs::petersen;
let (num_vertices, edges) = petersen();
// 10 vertices, 15 edges, 3-regular, MIS=4, VC=6, Matching=5, DomSet=3, χ=3
```

### Octahedron
```rust
use problemreductions::topology::small_graphs::octahedral;
let (num_vertices, edges) = octahedral();
// 6 vertices, 12 edges, K_{2,2,2}, Clique=3
```

### 3-SAT Instance (5 variables, 7 clauses)
```rust
let sat = Satisfiability::<i32>::new(
    5,
    vec![
        CNFClause::new(vec![1, 2, -3]),    // x1 ∨ x2 ∨ ¬x3
        CNFClause::new(vec![-1, 3, 4]),    // ¬x1 ∨ x3 ∨ x4
        CNFClause::new(vec![2, -4, 5]),    // x2 ∨ ¬x4 ∨ x5
        CNFClause::new(vec![-2, 3, -5]),   // ¬x2 ∨ x3 ∨ ¬x5
        CNFClause::new(vec![1, -3, 5]),    // x1 ∨ ¬x3 ∨ x5
        CNFClause::new(vec![-1, -2, 4]),   // ¬x1 ∨ ¬x2 ∨ x4
        CNFClause::new(vec![3, -4, -5]),   // x3 ∨ ¬x4 ∨ ¬x5
    ],
);
```
**Note:** After implementing, verify this has 2-8 satisfying assignments (not 0 and not all 32). If it has 0 solutions, flip one literal sign. If too many, add a clause.

### Petersen SpinGlass (frustrated, ±1 couplings)
```rust
use problemreductions::topology::small_graphs::petersen;
let (n, edges) = petersen();
// Alternating ±1 couplings → frustration on odd cycles
let couplings: Vec<((usize, usize), f64)> = edges.iter().enumerate()
    .map(|(i, &(u, v))| ((u, v), if i % 2 == 0 { 1.0 } else { -1.0 }))
    .collect();
let sg = SpinGlass::<SimpleGraph, f64>::new(n, couplings, vec![0.0; n]);
```

---

## Task 1: MIS → QUBO, ILP, MVC, MSP (4 files)

**Files:**
- Modify: `examples/reduction_maximumindependentset_to_qubo.rs`
- Modify: `examples/reduction_maximumindependentset_to_ilp.rs`
- Modify: `examples/reduction_maximumindependentset_to_minimumvertexcover.rs`
- Modify: `examples/reduction_maximumindependentset_to_maximumsetpacking.rs`

**Step 1: Update all 4 files**

In each file, replace the graph construction with:
```rust
use problemreductions::topology::small_graphs::petersen;

// Petersen graph: 10 vertices, 15 edges, 3-regular
let (num_vertices, edges) = petersen();
let is = MaximumIndependentSet::<SimpleGraph, i32>::new(num_vertices, edges.clone());
```

Replace the old:
```rust
let edges = vec![(0, 1), (1, 2), (2, 3)];
let is = MaximumIndependentSet::<SimpleGraph, i32>::new(4, edges.clone());
```

Update doc comments:
- `//! - Instance: Petersen graph with 10 vertices and 15 edges`
- `//! - Source: MaximumIndependentSet with maximum size 4`
- `//! - QUBO variables: 10` (for the QUBO example)

Update print statements to say "Petersen graph" instead of "path P4".

**Step 2: Run all 4 examples to verify**

```bash
cargo run --all-features --example reduction_maximumindependentset_to_qubo
cargo run --all-features --example reduction_maximumindependentset_to_ilp
cargo run --all-features --example reduction_maximumindependentset_to_minimumvertexcover
cargo run --all-features --example reduction_maximumindependentset_to_maximumsetpacking
```

Expected: Each prints solutions with MIS size 4. No panics.

**Step 3: Commit**

```bash
git add examples/reduction_maximumindependentset_*.rs
git commit -m "examples: use Petersen graph for MIS reductions"
```

---

## Task 2: MVC → ILP, QUBO, MIS, MSC (4 files)

**Files:**
- Modify: `examples/reduction_minimumvertexcover_to_ilp.rs`
- Modify: `examples/reduction_minimumvertexcover_to_qubo.rs`
- Modify: `examples/reduction_minimumvertexcover_to_maximumindependentset.rs`
- Modify: `examples/reduction_minimumvertexcover_to_minimumsetcovering.rs`

**Step 1: Update all 4 files**

Replace graph construction with:
```rust
use problemreductions::topology::small_graphs::petersen;

let (num_vertices, edges) = petersen();
let vc = MinimumVertexCover::<SimpleGraph, i32>::new(num_vertices, edges.clone());
```

Replace old C4 `vec![(0, 1), (1, 2), (2, 3), (0, 3)]` or K3 `vec![(0, 1), (1, 2), (0, 2)]`.

Update doc comments to reference Petersen, VC=6.

**Step 2: Run all 4 examples**

```bash
cargo run --all-features --example reduction_minimumvertexcover_to_ilp
cargo run --all-features --example reduction_minimumvertexcover_to_qubo
cargo run --all-features --example reduction_minimumvertexcover_to_maximumindependentset
cargo run --all-features --example reduction_minimumvertexcover_to_minimumsetcovering
```

Expected: VC size 6. No panics.

**Step 3: Commit**

```bash
git add examples/reduction_minimumvertexcover_*.rs
git commit -m "examples: use Petersen graph for MVC reductions"
```

---

## Task 3: Matching + DomSet → ILP, MSP (3 files)

**Files:**
- Modify: `examples/reduction_maximummatching_to_ilp.rs`
- Modify: `examples/reduction_maximummatching_to_maximumsetpacking.rs`
- Modify: `examples/reduction_minimumdominatingset_to_ilp.rs`

**Step 1: Update all 3 files**

For Matching (uses `unweighted` constructor):
```rust
use problemreductions::topology::small_graphs::petersen;

let (num_vertices, edges) = petersen();
let matching = MaximumMatching::<SimpleGraph, i32>::unweighted(num_vertices, edges.clone());
```

For DominatingSet:
```rust
use problemreductions::topology::small_graphs::petersen;

let (num_vertices, edges) = petersen();
let ds = MinimumDominatingSet::<SimpleGraph, i32>::new(num_vertices, edges.clone());
```

Update doc comments: Matching=5 (perfect), DomSet=3.

**Step 2: Run all 3 examples**

```bash
cargo run --all-features --example reduction_maximummatching_to_ilp
cargo run --all-features --example reduction_maximummatching_to_maximumsetpacking
cargo run --all-features --example reduction_minimumdominatingset_to_ilp
```

Expected: Matching size 5, DomSet size 3. No panics.

**Step 3: Commit**

```bash
git add examples/reduction_maximummatching_*.rs examples/reduction_minimumdominatingset_*.rs
git commit -m "examples: use Petersen graph for Matching and DomSet reductions"
```

---

## Task 4: Coloring + MaxCut (3 files)

**Files:**
- Modify: `examples/reduction_coloring_to_ilp.rs`
- Modify: `examples/reduction_coloring_to_qubo.rs`
- Modify: `examples/reduction_maxcut_to_spinglass.rs`

**Step 1: Update Coloring files**

Petersen has chromatic number 3, so `KColoring::<3, ...>` is correct:
```rust
use problemreductions::topology::small_graphs::petersen;

let (num_vertices, edges) = petersen();
let kc = KColoring::<3, SimpleGraph, i32>::new(num_vertices, edges.clone());
```

**IMPORTANT for `coloring_to_qubo`**: KColoring::<3> on Petersen creates a QUBO with 10×3 = 30 variables. BruteForce on 30 variables (2^30 ≈ 1B) is too slow. Either:
- (a) Use a smaller graph for coloring QUBO examples (e.g., `house()` — 5 vertices, χ=3, QUBO=15 vars), or
- (b) Accept slow runtime (~minutes).

**Recommended**: Use `house()` (5 vertices, 6 edges, χ=3) for `coloring_to_qubo` only. Keep Petersen for `coloring_to_ilp`.

**Step 1b: Update MaxCut file**

MaxCut with unit weights on Petersen:
```rust
use problemreductions::topology::small_graphs::petersen;

let (num_vertices, edges) = petersen();
let maxcut = MaxCut::<SimpleGraph, i32>::unweighted(num_vertices, edges.clone());
```

**Step 2: Run all 3 examples**

```bash
cargo run --all-features --example reduction_coloring_to_ilp
cargo run --all-features --example reduction_coloring_to_qubo
cargo run --all-features --example reduction_maxcut_to_spinglass
```

Expected: Coloring solutions with 3 colors. MaxCut solution. No panics. Verify `coloring_to_qubo` completes within ~2 minutes.

**Step 3: Commit**

```bash
git add examples/reduction_coloring_*.rs examples/reduction_maxcut_*.rs
git commit -m "examples: use Petersen graph for Coloring and MaxCut reductions"
```

---

## Task 5: MaximumClique → ILP (1 file)

**Files:**
- Modify: `examples/reduction_maximumclique_to_ilp.rs`

**Step 1: Update instance**

```rust
use problemreductions::topology::small_graphs::octahedral;

// Octahedron: 6 vertices, 12 edges, K_{2,2,2}, clique number = 3
let (num_vertices, edges) = octahedral();
let clique = MaximumClique::<SimpleGraph, i32>::new(num_vertices, edges.clone());
```

Update doc comments: "Octahedron graph (K_{2,2,2}) with 6 vertices and 12 edges, max clique size 3."

**Step 2: Run example**

```bash
cargo run --all-features --example reduction_maximumclique_to_ilp
```

Expected: Clique of size 3. No panics.

**Step 3: Commit**

```bash
git add examples/reduction_maximumclique_to_ilp.rs
git commit -m "examples: use octahedron for MaximumClique reduction"
```

---

## Task 6: Standalone Set Problems (3 files)

**Files:**
- Modify: `examples/reduction_maximumsetpacking_to_qubo.rs`
- Modify: `examples/reduction_maximumsetpacking_to_ilp.rs`
- Modify: `examples/reduction_minimumsetcovering_to_ilp.rs`

**Step 1: Update instances**

These are standalone set problems (not derived from graph reductions). Replace the trivial 3-set instances with 6 sets over 8 elements:

For MaximumSetPacking:
```rust
let sets = vec![
    vec![0, 1, 2],    // S0
    vec![2, 3, 4],    // S1 (overlaps S0 at 2)
    vec![4, 5, 6],    // S2 (overlaps S1 at 4)
    vec![6, 7, 0],    // S3 (overlaps S2 at 6, S0 at 0)
    vec![1, 3, 5],    // S4 (overlaps S0,S1,S2)
    vec![0, 4, 7],    // S5 (overlaps S0,S1,S3)
];
let sp = MaximumSetPacking::<i32>::new(sets.clone());
```

For MinimumSetCovering (same 6 sets, universe size 8):
```rust
let sets = vec![
    vec![0, 1, 2],
    vec![2, 3, 4],
    vec![4, 5, 6],
    vec![6, 7, 0],
    vec![1, 3, 5],
    vec![0, 4, 7],
];
let sc = MinimumSetCovering::<i32>::new(8, sets.clone());
```

**Step 2: Run all 3 examples**

```bash
cargo run --all-features --example reduction_maximumsetpacking_to_qubo
cargo run --all-features --example reduction_maximumsetpacking_to_ilp
cargo run --all-features --example reduction_minimumsetcovering_to_ilp
```

Expected: MSP finds max packing (disjoint sets), MSC finds min covering. No panics.

**IMPORTANT for `msp_to_qubo`**: SetPacking with 6 sets → QUBO with 6 variables. BruteForce on 6 vars is instant. Good.

**Step 3: Commit**

```bash
git add examples/reduction_maximumsetpacking_*.rs examples/reduction_minimumsetcovering_to_ilp.rs
git commit -m "examples: use 6-set instances for SetPacking and SetCovering"
```

---

## Task 7: SAT → MIS, Coloring, DomSet, kSAT (4 files)

**Files:**
- Modify: `examples/reduction_sat_to_maximumindependentset.rs`
- Modify: `examples/reduction_sat_to_coloring.rs`
- Modify: `examples/reduction_sat_to_minimumdominatingset.rs`
- Modify: `examples/reduction_sat_to_ksat.rs`

**Step 1: Update all 4 files**

Replace the SAT construction with the shared 3-SAT instance (see Shared Constants above). Use the exact same formula in all 4 files:
```rust
let sat = Satisfiability::<i32>::new(
    5,
    vec![
        CNFClause::new(vec![1, 2, -3]),
        CNFClause::new(vec![-1, 3, 4]),
        CNFClause::new(vec![2, -4, 5]),
        CNFClause::new(vec![-2, 3, -5]),
        CNFClause::new(vec![1, -3, 5]),
        CNFClause::new(vec![-1, -2, 4]),
        CNFClause::new(vec![3, -4, -5]),
    ],
);
```

Update doc comments to reference "5-variable, 7-clause 3-SAT formula".

**IMPORTANT**: For `sat_to_mis`, the target MIS graph has 7×3 = 21 vertices. BruteForce on 21 variables (2^21 ≈ 2M) is fast. For `sat_to_coloring`, the target has more variables (2×5 + 7×3 = 31 — too slow for BruteForce). If `sat_to_coloring` is too slow, reduce to 5 clauses instead of 7. For `sat_to_mds`, check the target variable count similarly.

Run each example and verify it completes within ~30 seconds. If any is too slow, trim the formula to fewer clauses for that specific example.

**Step 2: Run all 4 examples**

```bash
cargo run --all-features --example reduction_sat_to_maximumindependentset
cargo run --all-features --example reduction_sat_to_coloring
cargo run --all-features --example reduction_sat_to_minimumdominatingset
cargo run --all-features --example reduction_sat_to_ksat
```

Expected: SAT solutions found, reductions verified. No panics.

**Step 3: Commit**

```bash
git add examples/reduction_sat_*.rs
git commit -m "examples: use 5-variable 3-SAT instance for SAT reductions"
```

---

## Task 8: kSAT → QUBO (1 file)

**Files:**
- Modify: `examples/reduction_ksatisfiability_to_qubo.rs`

**Step 1: Update instance**

Use the same 3-SAT formula but as `KSatisfiability::<3, i32>`:
```rust
let clauses = vec![
    CNFClause::new(vec![1, 2, -3]),
    CNFClause::new(vec![-1, 3, 4]),
    CNFClause::new(vec![2, -4, 5]),
    CNFClause::new(vec![-2, 3, -5]),
    CNFClause::new(vec![1, -3, 5]),
    CNFClause::new(vec![-1, -2, 4]),
    CNFClause::new(vec![3, -4, -5]),
];
let ksat = KSatisfiability::<3, i32>::new(5, clauses);
```

**IMPORTANT**: kSAT → QUBO creates auxiliary variables. Check that the QUBO has ≤ ~25 variables. If too many, reduce to fewer clauses.

**Step 2: Run example**

```bash
cargo run --all-features --example reduction_ksatisfiability_to_qubo
```

Expected: QUBO solutions found, kSAT verified. No panics. Completes within ~1 minute.

**Step 3: Commit**

```bash
git add examples/reduction_ksatisfiability_to_qubo.rs
git commit -m "examples: use 5-variable 3-SAT instance for kSAT-to-QUBO"
```

---

## Task 9: SpinGlass ↔ QUBO ↔ MaxCut (3 files)

**Files:**
- Modify: `examples/reduction_spinglass_to_qubo.rs`
- Modify: `examples/reduction_qubo_to_spinglass.rs`
- Modify: `examples/reduction_spinglass_to_maxcut.rs`

**Step 1: Update SpinGlass → QUBO and SpinGlass → MaxCut**

Both start with a SpinGlass instance. Use the Petersen SpinGlass (see Shared Constants):
```rust
use problemreductions::topology::small_graphs::petersen;

let (n, edges) = petersen();
let couplings: Vec<((usize, usize), f64)> = edges.iter().enumerate()
    .map(|(i, &(u, v))| ((u, v), if i % 2 == 0 { 1.0 } else { -1.0 }))
    .collect();
let sg = SpinGlass::<SimpleGraph, f64>::new(n, couplings, vec![0.0; n]);
```

For `spinglass_to_maxcut`, use `i32` weights instead of `f64`:
```rust
let couplings: Vec<((usize, usize), i32)> = edges.iter().enumerate()
    .map(|(i, &(u, v))| ((u, v), if i % 2 == 0 { 1 } else { -1 }))
    .collect();
let sg = SpinGlass::<SimpleGraph, i32>::new(n, couplings, vec![0; n]);
```

**Step 1b: Update QUBO → SpinGlass**

Create a 10-variable QUBO directly. Use a sparse upper-triangular matrix on Petersen edges:
```rust
use problemreductions::topology::small_graphs::petersen;

let (n, edges) = petersen();
let mut matrix = vec![vec![0.0; n]; n];
// Diagonal: linear terms
for i in 0..n {
    matrix[i][i] = -1.0 + 0.2 * i as f64;
}
// Off-diagonal: quadratic terms on Petersen edges
for (idx, &(u, v)) in edges.iter().enumerate() {
    let (i, j) = if u < v { (u, v) } else { (v, u) };
    matrix[i][j] = if idx % 2 == 0 { 2.0 } else { -1.5 };
}
let qubo = QUBO::from_matrix(matrix.clone());
```

**Step 2: Run all 3 examples**

```bash
cargo run --all-features --example reduction_spinglass_to_qubo
cargo run --all-features --example reduction_qubo_to_spinglass
cargo run --all-features --example reduction_spinglass_to_maxcut
```

Expected: Solutions found, round-trip verified. No panics.

**Step 3: Commit**

```bash
git add examples/reduction_spinglass_*.rs examples/reduction_qubo_to_spinglass.rs
git commit -m "examples: use Petersen topology for SpinGlass/QUBO/MaxCut"
```

---

## Task 10: ILP → QUBO (1 file)

**Files:**
- Modify: `examples/reduction_ilp_to_qubo.rs`

**Step 1: Update instance**

Replace the trivial 3-variable ILP with a 6-variable binary knapsack:
```rust
let ilp = ILP::binary(
    6,
    vec![
        // Knapsack weight constraint: 3x0 + 2x1 + 5x2 + 4x3 + 2x4 + 3x5 ≤ 10
        LinearConstraint::le(
            vec![(0, 3.0), (1, 2.0), (2, 5.0), (3, 4.0), (4, 2.0), (5, 3.0)],
            10.0,
        ),
        // Category limit: x0 + x1 + x2 ≤ 2
        LinearConstraint::le(vec![(0, 1.0), (1, 1.0), (2, 1.0)], 2.0),
        // Category limit: x3 + x4 + x5 ≤ 2
        LinearConstraint::le(vec![(3, 1.0), (4, 1.0), (5, 1.0)], 2.0),
    ],
    vec![(0, 10.0), (1, 7.0), (2, 12.0), (3, 8.0), (4, 6.0), (5, 9.0)],
    ObjectiveSense::Maximize,
);
```

**IMPORTANT**: ILP → QUBO adds slack variables. Check the QUBO has ≤ ~25 variables. If too many, reduce the RHS of the knapsack constraint (fewer slack bits needed).

**Step 2: Run example**

```bash
cargo run --all-features --example reduction_ilp_to_qubo
```

Expected: QUBO solution extracts to a valid knapsack solution. No panics.

**Step 3: Commit**

```bash
git add examples/reduction_ilp_to_qubo.rs
git commit -m "examples: use 6-variable knapsack for ILP-to-QUBO"
```

---

## Task 11: CircuitSAT → SpinGlass (1 file)

**Files:**
- Modify: `examples/reduction_circuit_to_spinglass.rs`

**Step 1: Update instance**

Replace the single AND gate with a full adder circuit (1-bit addition with carry):
```rust
use problemreductions::models::specialized::Circuit;

// Full adder: inputs a, b, cin → outputs sum, cout
// sum = a XOR b XOR cin
// cout = (a AND b) OR (cin AND (a XOR b))
let circuit = Circuit::new(vec![
    // Intermediate: t = a XOR b
    Assignment::new(
        vec!["t".to_string()],
        BooleanExpr::xor(vec![BooleanExpr::var("a"), BooleanExpr::var("b")]),
    ),
    // sum = t XOR cin
    Assignment::new(
        vec!["sum".to_string()],
        BooleanExpr::xor(vec![BooleanExpr::var("t"), BooleanExpr::var("cin")]),
    ),
    // ab = a AND b
    Assignment::new(
        vec!["ab".to_string()],
        BooleanExpr::and(vec![BooleanExpr::var("a"), BooleanExpr::var("b")]),
    ),
    // cin_t = cin AND t
    Assignment::new(
        vec!["cin_t".to_string()],
        BooleanExpr::and(vec![BooleanExpr::var("cin"), BooleanExpr::var("t")]),
    ),
    // cout = ab OR cin_t
    Assignment::new(
        vec!["cout".to_string()],
        BooleanExpr::or(vec![BooleanExpr::var("ab"), BooleanExpr::var("cin_t")]),
    ),
]);
let circuit_sat = CircuitSAT::<i32>::new(circuit);
```

This gives ~7 variables (a, b, cin, t, sum, ab, cin_t, cout). BruteForce on 8 variables is instant.

**Step 2: Run example**

```bash
cargo run --all-features --example reduction_circuit_to_spinglass
```

Expected: SpinGlass solutions found. No panics.

**Step 3: Commit**

```bash
git add examples/reduction_circuit_to_spinglass.rs
git commit -m "examples: use full adder circuit for CircuitSAT-to-SpinGlass"
```

---

## Task 12: Factoring → Circuit, ILP (2 files)

**Files:**
- Modify: `examples/reduction_factoring_to_circuit.rs`
- Modify: `examples/reduction_factoring_to_ilp.rs`

**Step 1: Update instances**

Replace `Factoring::new(2, 2, 6)` and `Factoring::new(4, 4, 15)` with:
```rust
let factoring = Factoring::new(3, 3, 35);
// Factor 35 = 5 × 7, 3-bit factors, 6 binary variables
```

For `factoring_to_circuit.rs`: update the variable name format strings. The current code uses `format!("p{}", i + 1)` and `format!("q{}", i + 1)` which should still work for 3-bit factors.

Update doc comments: "Factor 35 = 5 × 7 (m=3 bits, n=3 bits)".

For `factoring_to_ilp.rs`: the ILPSolver is used (not BruteForce). This should handle 3×3 fine.

**Step 2: Run both examples**

```bash
cargo run --all-features --example reduction_factoring_to_circuit
cargo run --all-features --example reduction_factoring_to_ilp
```

Expected: Finds factors 5 and 7 (and 7 and 5). No panics.

**Step 3: Commit**

```bash
git add examples/reduction_factoring_*.rs
git commit -m "examples: use factoring 35=5×7 for Factoring reductions"
```

---

## Task 13: Regenerate JSON and Full Verification

**Files:**
- All `docs/paper/examples/*.json` and `*.result.json` (auto-generated)

**Step 1: Run all examples to regenerate JSON**

```bash
make examples
```

If no `make examples` target exists, run manually:
```bash
for ex in $(ls examples/reduction_*.rs | sed 's|examples/||;s|\.rs||'); do
    cargo run --all-features --example "$ex"
done
```

**Step 2: Run full test suite**

```bash
make test
```

Expected: All tests pass. The QUBO ground truth tests in `tests/data/qubo/` use different instances than the examples, so they should not be affected.

**Step 3: Run clippy**

```bash
make clippy
```

Expected: No warnings.

**Step 4: Verify JSON files updated**

```bash
git diff --stat docs/paper/examples/
```

Expected: All 60 JSON files (30 × `.json` + 30 × `.result.json`) show changes.

**Step 5: Commit generated files**

```bash
git add docs/paper/examples/
git commit -m "chore: regenerate example JSON with improved instances"
```

---

## Parallel Execution Groups

Tasks 1-12 are independent and can run in parallel. Task 13 depends on all others completing.

**Group A (graph, can run in parallel):** Tasks 1, 2, 3, 4, 5
**Group B (non-graph, can run in parallel):** Tasks 6, 7, 8, 9, 10, 11, 12
**Group C (verification, sequential after A+B):** Task 13
