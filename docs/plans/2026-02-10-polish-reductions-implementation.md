# Polish Reductions.typ Documentation - Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Polish reductions.typ by connecting math to code, adding theorem labels, expanding proofs, and creating 28 example files with JSON export.

**Architecture:** 5-pass approach: (1) Add theorem labels, (2) Enhance problem definitions, (3) Expand proofs + add links, (4) Create example files, (5) Verify compilation.

**Tech Stack:** Typst (docs/paper/reductions.typ), Rust (examples/), serde_json (JSON export)

**Design Reference:** `docs/plans/2026-02-10-polish-reductions-paper.md`

---

## Overview

This plan implements a comprehensive documentation polish consisting of:
- **Pass 1:** Add `<thm:*>` labels to all theorems for cross-referencing
- **Pass 2:** Enhance 15 problem definitions with field mappings and theorem links
- **Pass 3:** Expand trivial reduction proofs and add GitHub example links
- **Pass 4:** Create 28 example files with JSON export (split existing qubo_reductions.rs)
- **Pass 5:** Verify `make paper` compiles and all examples run

**Important Notes:**
- pkgref/ contains reference implementations (ProblemReductions.jl, UnitDiskMapping.jl, qubogen)
- Unit Disk Mapping already has export_petersen_mapping.rs (polished)
- Each example exports JSON to docs/paper/examples/ for paper code blocks

---

## PASS 1: Add Theorem Labels

### Task 1.1: Scan and catalog all theorems

**Files:**
- Read: `docs/paper/reductions.typ:312-940`

**Step 1: Extract theorem titles**

```bash
cd /Users/liujinguo/rcode/problemreductions
grep -n "^#theorem\[" docs/paper/reductions.typ
```

Expected: List of line numbers and theorem titles (e.g., "*(IS $arrow.l.r$ VC)*")

**Step 2: Create theorem-to-label mapping**

Create temporary file listing all theorems with proposed labels:
- IS ↔ VC → `<thm:is-to-vc>` and `<thm:vc-to-is>`
- IS → SetPacking → `<thm:is-to-setpacking>`
- SpinGlass ↔ QUBO → `<thm:spinglass-to-qubo>` and `<thm:qubo-to-spinglass>`
- etc.

Save to: `docs/paper/.theorem_labels.txt` (temporary scratch file)

**Step 3: Verify no duplicate labels**

Check for uniqueness in the mapping file.

Expected: All labels unique


### Task 1.2: Add labels to trivial reduction theorems

**Files:**
- Modify: `docs/paper/reductions.typ:314-370`

**Step 1: Add label to IS ↔ VC theorem**

Find the theorem block starting with `*(IS $arrow.l.r$ VC)*` and add label after the closing bracket:

```typst
#theorem[
  *(IS $arrow.l.r$ VC)* $S subset.eq V$ is independent iff $V backslash S$ is a vertex cover, with $|"IS"| + |"VC"| = |V|$. [_Problems:_ @def:independent-set, @def:vertex-cover.]
] <thm:is-to-vc>
```

**Step 2: Add label to IS → SetPacking theorem**

```typst
#theorem[
  *(IS $arrow.r$ Set Packing)* Construct $U = E$, $S_v = {e in E : v in e}$, $w(S_v) = w(v)$. Then $I$ is independent iff ${S_v : v in I}$ is a packing. [_Problems:_ @def:independent-set, @def:set-packing.]
] <thm:is-to-setpacking>
```

**Step 3: Add labels to remaining trivial reductions**

Continue adding labels to:
- VC → SetCovering: `<thm:vc-to-setcovering>`
- Matching → SetPacking: `<thm:matching-to-setpacking>`
- SpinGlass ↔ QUBO: `<thm:spinglass-to-qubo>` (bidirectional)

**Step 4: Commit trivial reduction labels**

```bash
git add docs/paper/reductions.typ
git commit -m "docs: add theorem labels to trivial reductions

Added <thm:*> labels to IS↔VC, IS→SetPacking, VC→SetCovering,
Matching→SetPacking, SpinGlass↔QUBO for cross-referencing"
```


### Task 1.3: Add labels to penalty-method QUBO theorems

**Files:**
- Modify: `docs/paper/reductions.typ:384-560`

**Step 1: Add label to IS → QUBO theorem**

```typst
#theorem[
  *(IS $arrow.r$ QUBO)* Given $G = (V, E)$ with weights $w$, construct upper-triangular $Q in RR^(n times n)$ with $Q_(i i) = -w_i$ and $Q_(i j) = P$ for $(i,j) in E$ ($i < j$), where $P = 1 + sum_i w_i$. Then minimizing $f(bold(x)) = sum_i Q_(i i) x_i + sum_(i<j) Q_(i j) x_i x_j$ is equivalent to maximizing the IS objective. [_Problems:_ @def:independent-set, @def:qubo.]
] <thm:is-to-qubo>
```

**Step 2: Add labels to remaining QUBO reductions**

Continue with:
- VC → QUBO: `<thm:vc-to-qubo>`
- KColoring → QUBO: `<thm:coloring-to-qubo>`
- SetPacking → QUBO: `<thm:setpacking-to-qubo>`
- 2-SAT → QUBO: `<thm:ksatisfiability-to-qubo>`
- Binary ILP → QUBO: `<thm:ilp-to-qubo>`

**Step 3: Commit penalty-method labels**

```bash
git add docs/paper/reductions.typ
git commit -m "docs: add theorem labels to penalty-method QUBO reductions

Added labels for IS→QUBO, VC→QUBO, Coloring→QUBO, SetPacking→QUBO,
KSatisfiability→QUBO, ILP→QUBO"
```


### Task 1.4: Add labels to non-trivial reduction theorems

**Files:**
- Modify: `docs/paper/reductions.typ:562-720`

**Step 1: Add labels to SAT reduction theorems**

```typst
#theorem[
  *(SAT $arrow.r$ IS)* @karp1972 Given CNF $phi$ with $m$ clauses, construct graph $G$ such that $phi$ is satisfiable iff $G$ has an IS of size $m$. [_Problems:_ @def:satisfiability, @def:independent-set.]
] <thm:sat-to-is>

#theorem[
  *(SAT $arrow.r$ 3-Coloring)* @garey1979 Given CNF $phi$, construct graph $G$ such that $phi$ is satisfiable iff $G$ is 3-colorable. [_Problems:_ @def:satisfiability, @def:coloring.]
] <thm:sat-to-coloring>

#theorem[
  *(SAT $arrow.r$ Dominating Set)* @garey1979 Given CNF $phi$ with $n$ variables and $m$ clauses, $phi$ is satisfiable iff the constructed graph has a dominating set of size $n$. [_Problems:_ @def:satisfiability, @def:dominating-set.]
] <thm:sat-to-dominatingset>

#theorem[
  *(SAT $arrow.l.r$ $k$-SAT)* @cook1971 @garey1979 Any SAT formula converts to $k$-SAT ($k >= 3$) preserving satisfiability. [_Problems:_ @def:satisfiability, @def:k-sat.]
] <thm:sat-to-ksat>
```

**Step 2: Add labels to CircuitSAT and Factoring theorems**

```typst
#theorem[
  *(CircuitSAT $arrow.r$ Spin Glass)* @whitfield2012 @lucas2014 Each gate maps to a gadget whose ground states encode valid I/O. [_Problems:_ @def:circuit-sat, @def:spin-glass.]
] <thm:circuit-to-spinglass>

#theorem[
  *(Factoring $arrow.r$ Circuit-SAT)* An array multiplier with output constrained to $N$ is satisfiable iff $N$ factors within bit bounds. _(Folklore; no canonical reference.)_ [_Problems:_ @def:factoring, @def:circuit-sat.]
] <thm:factoring-to-circuit>
```

**Step 3: Add labels to SpinGlass ↔ MaxCut and ILP theorems**

```typst
#theorem[
  *(Spin Glass $arrow.l.r$ Max-Cut)* @barahona1982 @lucas2014 Ground states of Ising models correspond to maximum cuts. [_Problems:_ @def:spin-glass, @def:max-cut.]
] <thm:spinglass-to-maxcut>

#theorem[
  *(Coloring $arrow.r$ ILP)* The $k$-coloring problem reduces to binary ILP with $|V| dot k$ variables and $|V| + |E| dot k$ constraints. [_Problems:_ @def:coloring, @def:ilp.]
] <thm:coloring-to-ilp>

#theorem[
  *(Factoring $arrow.r$ ILP)* Integer factorization reduces to binary ILP using McCormick linearization with $O(m n)$ variables and constraints. [_Problems:_ @def:factoring, @def:ilp.]
] <thm:factoring-to-ilp>
```

**Step 4: Add label to Unit Disk Mapping theorem**

```typst
#theorem[
  *(IS $arrow.r$ GridGraph IS)* @nguyen2023 Any MIS problem on a general graph $G$ can be reduced to MIS on a unit disk graph (King's subgraph) with at most quadratic overhead in the number of vertices. [_Problem:_ @def:independent-set.]
] <thm:is-to-gridgraph>
```

**Step 5: Commit non-trivial reduction labels**

```bash
git add docs/paper/reductions.typ
git commit -m "docs: add theorem labels to non-trivial reductions

Added labels for SAT→IS, SAT→Coloring, SAT→DominatingSet, SAT↔KSAT,
CircuitSAT→SpinGlass, Factoring→Circuit, SpinGlass↔MaxCut,
Coloring→ILP, Factoring→ILP, IS→GridGraph"
```


### Task 1.5: Verify all theorem labels added

**Step 1: Count theorems and labels**

```bash
cd /Users/liujinguo/rcode/problemreductions
echo "Theorems:" && grep -c "^#theorem\[" docs/paper/reductions.typ
echo "Labels:" && grep -c "] <thm:" docs/paper/reductions.typ
```

Expected: Same count for both (approximately 28 theorems)

**Step 2: Check for label format consistency**

```bash
grep "] <thm:" docs/paper/reductions.typ | sed 's/.*<thm:\(.*\)>/\1/' | sort
```

Expected: All labels use lowercase with hyphens, no duplicates

**Step 3: Commit verification notes**

Update `.theorem_labels.txt` with final mapping, commit as documentation.

```bash
git add docs/paper/.theorem_labels.txt
git commit -m "docs: add theorem label mapping for reference"
```

---

## PASS 2: Enhance Problem Definitions

### Task 2.1: Enhance Independent Set definition

**Files:**
- Modify: `docs/paper/reductions.typ:65-78`

**Step 1: Add field mapping paragraph after struct**

Find the Independent Set definition and add after the Rust struct:

```typst
#definition("Independent Set (IS)")[
  Given $G = (V, E)$ with vertex weights $w: V -> RR$, find $S subset.eq V$ maximizing $sum_(v in S) w(v)$ such that no two vertices in $S$ are adjacent: $forall u, v in S: (u, v) in.not E$.

  ```rust
  pub struct IndependentSet<W = i32> {
      graph: UnGraph<(), ()>,  // The underlying graph
      weights: Vec<W>,         // Weights for each vertex
  }
  ```

  Where `graph` represents $G = (V, E)$ with vertices indexed $0..n-1$, and `weights` stores vertex weights $w: V -> RR$ indexed by vertex ID. The solution is a subset $S subset.eq V$ represented as a `Vec<usize>` of vertex indices.

  _Implemented reductions:_ IS→SetPacking (@thm:is-to-setpacking), IS→QUBO (@thm:is-to-qubo), IS→ILP (@thm:is-to-ilp), VC→IS (@thm:is-to-vc), SAT→IS (@thm:sat-to-is).
] <def:independent-set>
```

**Step 2: Commit Independent Set enhancement**

```bash
git add docs/paper/reductions.typ
git commit -m "docs: enhance Independent Set definition with field mappings

Added field mapping paragraph and replaced problem links with
theorem references for IS definition"
```


### Task 2.2: Enhance Vertex Cover definition

**Files:**
- Modify: `docs/paper/reductions.typ:80-93`

**Step 1: Add field mapping and theorem links**

```typst
#definition("Vertex Cover (VC)")[
  Given $G = (V, E)$ with vertex weights $w: V -> RR$, find $S subset.eq V$ minimizing $sum_(v in S) w(v)$ such that every edge has at least one endpoint in $S$: $forall (u, v) in E: u in S or v in S$.

  ```rust
  pub struct VertexCovering<W = i32> {
      graph: UnGraph<(), ()>,  // The underlying graph
      weights: Vec<W>,         // Weights for each vertex
  }
  ```

  Where `graph` represents $G = (V, E)$ with vertices indexed $0..n-1$, and `weights` stores vertex weights $w: V -> RR$ indexed by vertex ID. The solution is a subset $S subset.eq V$ represented as a `Vec<usize>` of vertex indices covering all edges.

  _Implemented reductions:_ VC→IS (@thm:is-to-vc), VC→SetCovering (@thm:vc-to-setcovering), VC→QUBO (@thm:vc-to-qubo), VC→ILP (@thm:vc-to-ilp), IS→VC (@thm:is-to-vc).
] <def:vertex-cover>
```

**Step 2: Commit Vertex Cover enhancement**

```bash
git add docs/paper/reductions.typ
git commit -m "docs: enhance Vertex Cover definition

Added field mapping paragraph and theorem references"
```


### Task 2.3: Enhance remaining graph problem definitions

**Files:**
- Modify: `docs/paper/reductions.typ:95-153`

**Step 1: Enhance Max-Cut, Graph Coloring, Dominating Set, Matching definitions**

For each definition:
1. Add field mapping paragraph after struct
2. Replace "Reduces to/from" with "Implemented reductions" using theorem labels
3. Keep existing struct code block

**Step 2: Commit graph problem enhancements**

```bash
git add docs/paper/reductions.typ
git commit -m "docs: enhance graph problem definitions

Added field mappings and theorem references for Max-Cut, Coloring,
DominatingSet, Matching, Unit Disk Graph"
```


### Task 2.4: Enhance set problem definitions

**Files:**
- Modify: `docs/paper/reductions.typ:155-184`

**Step 1: Enhance Set Packing and Set Covering**

Add field mapping paragraphs:
- Set Packing: Explain `sets` as collection, `weights` as set weights
- Set Covering: Explain `universe_size`, `sets`, `weights`

**Step 2: Commit set problem enhancements**

```bash
git add docs/paper/reductions.typ
git commit -m "docs: enhance set problem definitions

Added field mappings and theorem references for SetPacking and SetCovering"
```


### Task 2.5: Enhance optimization problem definitions

**Files:**
- Modify: `docs/paper/reductions.typ:186-242`

**Step 1: Enhance Spin Glass, QUBO, ILP definitions**

- SpinGlass: Explain `num_spins`, `interactions` (J_ij), `fields` (h_i)
- QUBO: Explain `num_vars`, `matrix` (upper triangular Q)
- ILP: Explain `num_vars`, `bounds`, `constraints`, `objective`, `sense`

**Step 2: Commit optimization problem enhancements**

```bash
git add docs/paper/reductions.typ
git commit -m "docs: enhance optimization problem definitions

Added field mappings and theorem references for SpinGlass, QUBO, ILP"
```


### Task 2.6: Enhance satisfiability problem definitions

**Files:**
- Modify: `docs/paper/reductions.typ:244-310`

**Step 1: Enhance SAT, K-SAT, Circuit-SAT, Factoring definitions**

- SAT: Explain `num_vars`, `clauses` (CNFClause), `weights`
- K-SAT: Similar to SAT but with K literals per clause
- Circuit-SAT: Explain `circuit`, `variables`, `weights`
- Factoring: Explain `m`, `n`, `target`

**Step 2: Commit satisfiability problem enhancements**

```bash
git add docs/paper/reductions.typ
git commit -m "docs: enhance satisfiability problem definitions

Added field mappings and theorem references for SAT, K-SAT,
CircuitSAT, Factoring"
```


### Task 2.7: Verify all problem definitions enhanced

**Step 1: Count field mapping paragraphs**

```bash
grep -c "Where \`" docs/paper/reductions.typ
```

Expected: 15 (one per problem definition)

**Step 2: Check all use theorem references**

```bash
grep "_Implemented reductions:_" docs/paper/reductions.typ | wc -l
```

Expected: 15

**Step 3: Commit verification checkpoint**

```bash
git commit --allow-empty -m "checkpoint: Pass 2 complete - all problem definitions enhanced"
```

---

## PASS 3: Expand Proofs and Add Example Links

### Task 3.1: Expand trivial reduction proofs

**Files:**
- Modify: `docs/paper/reductions.typ:316-370`

**Step 1: Expand IS ↔ VC proof**

Find the proof block and add variable mapping section at end:

```typst
#proof[
  ($arrow.r.double$) If $S$ is independent, for any $(u, v) in E$, at most one endpoint lies in $S$, so $V backslash S$ covers all edges. ($arrow.l.double$) If $C$ is a cover, for any $u, v in V backslash C$, $(u, v) in.not E$, so $V backslash C$ is independent.

  _Variable mapping:_ Given IS instance $(G, w)$, create VC instance $(G, w)$ with identical graph and weights. Solution extraction: for VC solution $C$, return $S = V backslash C$. The complement operation preserves optimality since $|S| + |C| = |V|$ is constant.
]
```

**Step 2: Expand remaining trivial proofs**

Add variable mapping sections to:
- IS → SetPacking: Explain edge set mapping
- VC → SetCovering: Explain edge coverage mapping
- Matching → SetPacking: Explain endpoint mapping
- SpinGlass ↔ QUBO: Already has formula expansion, just verify

**Step 3: Commit expanded trivial proofs**

```bash
git add docs/paper/reductions.typ
git commit -m "docs: expand trivial reduction proofs with variable mappings

Added explicit variable mapping explanations to IS↔VC, IS→SetPacking,
VC→SetCovering, Matching→SetPacking proofs"
```


### Task 3.2: Add GitHub links to trivial reductions

**Files:**
- Modify: `docs/paper/reductions.typ:316-370`

**Step 1: Add example link after IS ↔ VC proof**

After the proof block, before the code example:

```typst
See [reduction example](https://github.com/CodingThrust/problem-reductions/blob/main/examples/reduction_is_to_vc.rs).
```

**Step 2: Add links to all trivial reductions**

Continue for:
- IS → SetPacking: `examples/reduction_is_to_setpacking.rs`
- VC → SetCovering: `examples/reduction_vc_to_setcovering.rs`
- Matching → SetPacking: `examples/reduction_matching_to_setpacking.rs`
- SpinGlass ↔ QUBO: `examples/reduction_spinglass_to_qubo.rs` and `reduction_qubo_to_spinglass.rs`

**Step 3: Commit GitHub links**

```bash
git add docs/paper/reductions.typ
git commit -m "docs: add GitHub example links to trivial reductions"
```


### Task 3.3: Add GitHub links to QUBO reductions

**Files:**
- Modify: `docs/paper/reductions.typ:384-560`

**Step 1: Add links after each QUBO reduction proof**

After each proof/code example:
- IS → QUBO: `examples/reduction_is_to_qubo.rs`
- VC → QUBO: `examples/reduction_vc_to_qubo.rs`
- KColoring → QUBO: `examples/reduction_coloring_to_qubo.rs`
- SetPacking → QUBO: `examples/reduction_setpacking_to_qubo.rs`
- 2-SAT → QUBO: `examples/reduction_ksatisfiability_to_qubo.rs`
- Binary ILP → QUBO: `examples/reduction_ilp_to_qubo.rs`

**Step 2: Commit QUBO reduction links**

```bash
git add docs/paper/reductions.typ
git commit -m "docs: add GitHub example links to QUBO reductions"
```


### Task 3.4: Add GitHub links to non-trivial reductions

**Files:**
- Modify: `docs/paper/reductions.typ:562-770`

**Step 1: Add links to SAT reduction theorems**

- SAT → IS: `examples/reduction_sat_to_is.rs`
- SAT → 3-Coloring: `examples/reduction_sat_to_coloring.rs`
- SAT → Dominating Set: `examples/reduction_sat_to_dominatingset.rs`
- SAT ↔ K-SAT: `examples/reduction_sat_to_ksat.rs`

**Step 2: Add links to remaining reductions**

- CircuitSAT → SpinGlass: `examples/reduction_circuit_to_spinglass.rs`
- Factoring → Circuit: `examples/reduction_factoring_to_circuit.rs`
- SpinGlass ↔ MaxCut: `examples/reduction_spinglass_to_maxcut.rs` and `reduction_maxcut_to_spinglass.rs`
- Coloring → ILP: `examples/reduction_coloring_to_ilp.rs`
- Factoring → ILP: `examples/reduction_factoring_to_ilp.rs`

**Step 3: Add link to Unit Disk Mapping**

After the Unit Disk Mapping theorem proof:

```typst
See [unit disk mapping example](https://github.com/CodingThrust/problem-reductions/blob/main/examples/export_petersen_mapping.rs).
```

**Step 4: Commit non-trivial reduction links**

```bash
git add docs/paper/reductions.typ
git commit -m "docs: add GitHub example links to non-trivial reductions

Added links for SAT, CircuitSAT, Factoring, SpinGlass/MaxCut, ILP
reductions and Unit Disk Mapping"
```


### Task 3.5: Verify all theorems have example links

**Step 1: Count GitHub links**

```bash
grep -c "See \[.*example\](https://github.com" docs/paper/reductions.typ
```

Expected: 28+ (one per reduction theorem)

**Step 2: Verify link format consistency**

```bash
grep "See \[.*example\]" docs/paper/reductions.typ
```

Expected: All use same format with `/blob/main/examples/`

**Step 3: Commit verification checkpoint**

```bash
git commit --allow-empty -m "checkpoint: Pass 3 complete - all proofs expanded and linked"
```

---

## PASS 4: Create Example Files

### Task 4.1: Split qubo_reductions.rs into separate files

**Files:**
- Read: `examples/qubo_reductions.rs`
- Create: 6 new files in `examples/`

**Step 1: Create reduction_is_to_qubo.rs**

Extract the `demo_independent_set()` function and create standalone example following the template from design doc. Include:
- Detailed docstring with mathematical equivalence
- Problem transformation metrics output
- JSON export to `docs/paper/examples/is_to_qubo.json`

**Step 2: Test the example compiles and runs**

```bash
cargo check --example reduction_is_to_qubo
cargo run --example reduction_is_to_qubo
```

Expected: Compiles, runs, outputs metrics, exports JSON

**Step 3: Commit IS → QUBO example**

```bash
git add examples/reduction_is_to_qubo.rs
git commit -m "feat: add IS→QUBO reduction example with JSON export

Extracted from qubo_reductions.rs, added detailed docstring
and JSON export for paper integration"
```

**Step 4: Repeat for remaining 5 QUBO examples**

Create in order:
- `reduction_vc_to_qubo.rs` from `demo_vertex_covering()`
- `reduction_coloring_to_qubo.rs` from `demo_coloring()`
- `reduction_setpacking_to_qubo.rs` from `demo_set_packing()`
- `reduction_ksatisfiability_to_qubo.rs` from `demo_ksat()`
- `reduction_ilp_to_qubo.rs` from `demo_ilp()`

Commit each separately.

**Step 5: Remove or rename original qubo_reductions.rs**

```bash
# Option: rename as tutorial
git mv examples/qubo_reductions.rs examples/tutorial_qubo_reductions.rs
# Or option: remove if redundant
# git rm examples/qubo_reductions.rs

git commit -m "refactor: rename qubo_reductions.rs to tutorial

Separated into individual reduction examples, keeping original
as comprehensive tutorial"
```


### Task 4.2: Create trivial reduction examples (IS↔VC, SetPacking, etc.)

**Files:**
- Create: `examples/reduction_is_to_vc.rs`
- Create: `examples/reduction_vc_to_is.rs`
- Create: `examples/reduction_is_to_setpacking.rs`
- Create: `examples/reduction_matching_to_setpacking.rs`
- Create: `examples/reduction_vc_to_setcovering.rs`
- Create: `examples/reduction_spinglass_to_qubo.rs` (if not already created)
- Create: `examples/reduction_qubo_to_spinglass.rs`
- Create: `examples/reduction_spinglass_to_maxcut.rs`
- Create: `examples/reduction_maxcut_to_spinglass.rs`

**Step 1: Create reduction_is_to_vc.rs**

Use UnitDiskMapping.jl's 5-vertex demo graph or Petersen graph. Follow template with:
- Docstring explaining complement relationship
- Transformation metrics
- JSON export with graph structure and solutions

```rust
//! # Independent Set to Vertex Cover Reduction
//!
//! ## Mathematical Equivalence
//! S ⊆ V is an independent set iff V \ S is a vertex cover.
//! Proof: If S is independent, no edge has both endpoints in S,
//! so every edge has at least one endpoint in V \ S.
//!
//! ## This Example
//! Demonstrates the complement relationship using the Petersen graph:
//! - Instance: Petersen graph (10 vertices, 15 edges)
//! - Maximum IS size: 4
//! - Minimum VC size: 6 (complement property: 4 + 6 = 10)
//! - Reference: Based on UnitDiskMapping.jl Petersen example
//!
//! ## Output
//! Exports `docs/paper/examples/is_to_vc.json` for use in paper code blocks.
//!
//! See docs/paper/reductions.typ for the full reduction specification.

use problemreductions::prelude::*;
use serde::Serialize;
use std::fs;

#[derive(Serialize)]
struct ExampleData {
    source_problem: String,
    target_problem: String,
    graph_vertices: usize,
    graph_edges: Vec<(usize, usize)>,
    source_size: usize,
    target_size: usize,
    source_solution: Vec<usize>,
    target_solution: Vec<usize>,
}

fn main() {
    // Petersen graph (10 vertices, 15 edges)
    let edges = vec![
        (0, 1), (1, 2), (2, 3), (3, 4), (4, 0),  // outer pentagon
        (5, 7), (7, 9), (9, 6), (6, 8), (8, 5),  // inner star
        (0, 5), (1, 6), (2, 7), (3, 8), (4, 9),  // spokes
    ];
    let is = IndependentSet::<i32>::new(10, edges.clone());

    println!("\n=== Problem Transformation ===");
    println!("Source: {} with {} variables", "IndependentSet", is.num_variables());

    // Reduce to VC
    let reduction = ReduceTo::<VertexCovering<i32>>::reduce_to(&is);
    let vc = reduction.target_problem();
    println!("Target: {} with {} variables", "VertexCovering", vc.num_variables());

    // Solve
    let solver = BruteForce::new();
    let vc_solutions = solver.find_best(vc);
    println!("\n=== Solution ===");
    println!("Target solutions found: {}", vc_solutions.len());

    // Extract
    let is_solution = reduction.extract_solution(&vc_solutions[0]);
    println!("Source solution: {:?}", is_solution);

    let size = is.solution_size(&is_solution);
    println!("Solution size: {:?}", size);
    assert!(size.is_valid);
    println!("\n✓ Reduction verified successfully");

    // Export JSON
    let example_data = ExampleData {
        source_problem: "IndependentSet".to_string(),
        target_problem: "VertexCovering".to_string(),
        graph_vertices: 10,
        graph_edges: edges,
        source_size: is.num_variables(),
        target_size: vc.num_variables(),
        source_solution: is_solution,
        target_solution: vc_solutions[0].clone(),
    };

    fs::create_dir_all("docs/paper/examples").unwrap();
    let json = serde_json::to_string_pretty(&example_data).unwrap();
    fs::write("docs/paper/examples/is_to_vc.json", json).unwrap();
    println!("  Exported: docs/paper/examples/is_to_vc.json");
}
```

**Step 2: Test and commit**

```bash
cargo check --example reduction_is_to_vc
cargo run --example reduction_is_to_vc
git add examples/reduction_is_to_vc.rs
git commit -m "feat: add IS→VC reduction example"
```

**Step 3: Create remaining trivial examples**

Follow same pattern for VC→IS, IS→SetPacking, etc. Commit each separately.


### Task 4.3: Create ILP reduction examples

**Files:**
- Create 9 ILP reduction examples in `examples/`

Create following the same pattern:
- `reduction_is_to_ilp.rs`
- `reduction_vc_to_ilp.rs`
- `reduction_matching_to_ilp.rs`
- `reduction_setpacking_to_ilp.rs`
- `reduction_setcovering_to_ilp.rs`
- `reduction_dominatingset_to_ilp.rs`
- `reduction_clique_to_ilp.rs`
- `reduction_coloring_to_ilp.rs` (already may exist)
- `reduction_factoring_to_ilp.rs` (already may exist)

Each should:
- Use small instances (5-10 variables)
- Include `--features ilp` note in docstring
- Export ILP constraints to JSON for paper

Commit each separately.


### Task 4.4: Create SAT and non-trivial reduction examples

**Files:**
- Create remaining examples in `examples/`

Create:
- `reduction_sat_to_is.rs` - use 3-4 variable SAT formula
- `reduction_sat_to_coloring.rs` - small SAT to coloring
- `reduction_sat_to_dominatingset.rs` - small SAT instance
- `reduction_sat_to_ksat.rs` - SAT to 3-SAT conversion
- `reduction_circuit_to_spinglass.rs` - small circuit
- `reduction_factoring_to_circuit.rs` - factor 6 = 2×3

Each should reference pkgref/ instances where applicable and export JSON.

Commit each separately.


### Task 4.5: Verify all 28 examples compile and run

**Step 1: Test all examples compile**

```bash
cd /Users/liujinguo/rcode/problemreductions
cargo check --examples
```

Expected: All examples compile without errors

**Step 2: Run all examples and verify JSON output**

```bash
for example in examples/reduction_*.rs; do
    name=$(basename "$example" .rs)
    echo "Running $name..."
    cargo run --example "$name" || echo "FAILED: $name"
done

ls -lh docs/paper/examples/*.json
```

Expected: All examples run, JSON files created

**Step 3: Commit verification script**

Create `scripts/test_examples.sh`:

```bash
#!/bin/bash
set -e
cd "$(dirname "$0")/.."
echo "Testing all reduction examples..."
for example in examples/reduction_*.rs; do
    name=$(basename "$example" .rs)
    cargo run --quiet --example "$name"
done
echo "✓ All examples passed"
```

```bash
chmod +x scripts/test_examples.sh
git add scripts/test_examples.sh
git commit -m "test: add example verification script"
```

---

## PASS 5: Final Verification

### Task 5.1: Verify paper compiles

**Step 1: Build the paper**

```bash
cd /Users/liujinguo/rcode/problemreductions
make paper
```

Expected: Typst compiles without errors, generates PDF

**Step 2: Check for broken references**

```bash
grep -n "@thm:" docs/paper/reductions.typ | grep -v "^[0-9]*:#theorem"
```

Expected: All theorem references resolve (no broken links)

**Step 3: Commit if fixes needed**

If any issues found, fix and commit:

```bash
git add docs/paper/reductions.typ
git commit -m "fix: resolve broken theorem references"
```


### Task 5.2: Run full test suite

**Step 1: Run Rust tests**

```bash
make test
```

Expected: All tests pass

**Step 2: Run clippy**

```bash
make clippy
```

Expected: No warnings

**Step 3: Run all examples**

```bash
./scripts/test_examples.sh
```

Expected: All examples run successfully


### Task 5.3: Generate final checklist report

**Step 1: Verify success criteria**

Create `docs/paper/VERIFICATION.md`:

```markdown
# Reductions.typ Polish - Verification Report

Date: 2026-02-10

## Success Criteria

- [x] 15 problem definitions have field mapping paragraphs
- [x] All problem definitions link to theorem labels (not problem definitions)
- [x] 28 theorems have labels and GitHub example links
- [x] Trivial reduction proofs explain variable mappings explicitly
- [x] 28 example files created with detailed docstrings
- [x] All examples use reference package instances where applicable
- [x] All examples export JSON to `docs/paper/examples/`
- [x] `docs/paper/examples/` added to `.gitignore` (already done)
- [x] Existing `qubo_reductions.rs` split into 6 separate files
- [x] `make paper` compiles successfully
- [x] All example files compile and run successfully

## Statistics

- Problem definitions enhanced: 15
- Theorems labeled: 28
- Example files created: 28
- JSON exports: 28
- Total commits: ~50-60

## Files Modified

- `docs/paper/reductions.typ` - main paper file
- `examples/` - 28 new example files
- `examples/qubo_reductions.rs` - renamed to tutorial
- `.gitignore` - already updated

## Next Steps

- Review generated PDF for formatting
- Verify all GitHub links work after PR merge
- Consider adding CI check for example JSON generation
```

**Step 2: Commit verification report**

```bash
git add docs/paper/VERIFICATION.md
git commit -m "docs: add verification report for reductions.typ polish"
```


### Task 5.4: Final cleanup and summary commit

**Step 1: Remove temporary files**

```bash
rm -f docs/paper/.theorem_labels.txt
```

**Step 2: Create summary commit**

```bash
git commit --allow-empty -m "feat: complete reductions.typ documentation polish

Implemented 5-pass documentation enhancement:

Pass 1: Added theorem labels (<thm:*>) to all 28 reduction theorems
Pass 2: Enhanced 15 problem definitions with field mappings and theorem links
Pass 3: Expanded trivial reduction proofs and added GitHub example links
Pass 4: Created 28 standalone example files with JSON export
Pass 5: Verified compilation and all examples run successfully

All examples follow consistent format:
- Detailed docstrings with mathematical context
- Transformation metrics output
- JSON export to docs/paper/examples/
- Based on reference package instances where applicable

See docs/paper/VERIFICATION.md for complete checklist."
```


### Task 5.5: Push and create PR (if in worktree)

**Step 1: Push changes**

```bash
git push origin HEAD
```

**Step 2: Create PR**

```bash
gh pr create --title "Polish reductions.typ documentation" \
  --body "Implements design from docs/plans/2026-02-10-polish-reductions-paper.md

## Changes
- Added theorem labels to all 28 reductions for cross-referencing
- Enhanced 15 problem definitions with field mappings
- Expanded trivial reduction proofs with variable mapping explanations
- Created 28 standalone example files with JSON export
- Split qubo_reductions.rs into 6 separate files
- All examples reference pkgref/ instances where applicable

## Verification
- ✓ make paper compiles successfully
- ✓ All 28 examples compile and run
- ✓ JSON exports generated
- ✓ All tests pass
- ✓ No clippy warnings

See docs/paper/VERIFICATION.md for complete checklist."
```

Expected: PR created and ready for review

---

## Notes

**Reference Package Usage:**
- pkgref/UnitDiskMapping.jl/examples/ - Petersen graph, 5-vertex demo
- pkgref/qubogen/tests/ - QUBO test cases with known matrices
- pkgref/ProblemReductions.jl/examples/ - Factoring example

**JSON Export Format:**
Each example exports structured data including:
- Problem names and sizes
- Input instance (graph edges, formulas, matrices)
- Solutions (source and target)
- Verification results

**Commit Strategy:**
- Small, focused commits (one enhancement per commit)
- ~50-60 commits total across 5 passes
- Checkpoint commits after each pass
- Final summary commit

**Testing:**
- Verify each example individually as created
- Run full suite at end
- Check paper compiles after each pass

