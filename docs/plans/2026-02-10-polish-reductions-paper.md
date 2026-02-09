# Polish reductions.typ Documentation

**Date:** 2026-02-10
**Status:** Design validated, ready for implementation

## Objectives

1. Connect mathematical symbols to program fields in problem definitions
2. Link from problem definitions to reduction theorems (not other problems)
3. Explain trivial reductions explicitly with variable mappings
4. Create standalone example files for all 28 reductions with GitHub links

## Design

### 1. Problem Definition Enhancement

For each of the 15 problem definitions in Section 2, add two modifications:

**A) Field Mapping Paragraph**

After the Rust struct, add a paragraph explaining the correspondence between mathematical notation and struct fields:

```typst
#definition("Independent Set (IS)")[
  Given $G = (V, E)$ with vertex weights $w: V -> RR$, find $S subset.eq V$ maximizing...

  ```rust
  pub struct IndependentSet<W = i32> {
      graph: UnGraph<(), ()>,
      weights: Vec<W>,
  }
  ```

  Where `graph` represents $G = (V, E)$ with vertices indexed $0..n-1$, and `weights` stores vertex weights $w: V -> RR$ indexed by vertex ID. The solution is a subset $S subset.eq V$ represented as a `Vec<usize>` of vertex indices.

  _Implemented reductions:_ IS→SetPacking (@thm:is-to-setpacking), IS→QUBO (@thm:is-to-qubo), VC→IS (@thm:vc-to-is), SAT→IS (@thm:sat-to-is), SetPacking→IS (@thm:setpacking-to-is).
] <def:independent-set>
```

**B) Link to Theorems Instead of Problems**

Replace current:
```typst
_Reduces to:_ Set Packing (@def:set-packing), QUBO (@def:qubo).
_Reduces from:_ Vertex Cover (@def:vertex-cover), SAT (@def:satisfiability).
```

With:
```typst
_Implemented reductions:_ IS→SetPacking (@thm:is-to-setpacking), IS→QUBO (@thm:is-to-qubo), VC→IS (@thm:vc-to-is), SAT→IS (@thm:sat-to-is).
```

### 2. Theorem Enhancement

**A) Add Labels to All Theorems**

Every reduction theorem gets a label for cross-referencing:

```typst
#theorem[
  *(IS $arrow.l.r$ VC)* ...
] <thm:is-to-vc>

#theorem[
  *(IS $arrow.r$ Set Packing)* ...
] <thm:is-to-setpacking>
```

Label format: `<thm:source-to-target>` using lowercase problem names with hyphens.

**B) Expand Trivial Reduction Proofs**

For trivial reductions (complement, isomorphism), add explicit variable mapping explanation:

```typst
#theorem[
  *(IS $arrow.l.r$ VC)* $S subset.eq V$ is independent iff $V backslash S$ is a vertex cover, with $|"IS"| + |"VC"| = |V|$.
] <thm:is-to-vc>

#proof[
  ($arrow.r.double$) If $S$ is independent, no edge has both endpoints in $S$, so every edge has at least one endpoint in $V backslash S$, making it a cover.

  ($arrow.l.double$) If $C$ is a cover, for any $u, v in V backslash C$, edge $(u,v)$ cannot exist (else uncovered), so $V backslash C$ is independent.

  _Variable mapping:_ Given IS instance $(G, w)$, create VC instance $(G, w)$ with identical graph and weights. Solution extraction: for VC solution $C$, return $S = V backslash C$. The complement operation preserves optimality since $|S| + |C| = |V|$ is constant.
]
```

**C) Add GitHub Links to Examples**

After each proof or embedded example, add:

```typst
See [reduction example](https://github.com/CodingThrust/problem-reductions/blob/main/examples/reduction_is_to_vc.rs).
```

### 3. Example Files Creation

Create 28 example files in `examples/` directory with flat naming structure:

#### Example File Template

```rust
//! # [Source] to [Target] Reduction
//!
//! ## Mathematical Equivalence
//! [Explain the mathematical relationship and why it works - 2-4 sentences]
//! [Reference to the mathematical proof if helpful]
//!
//! ## This Example
//! [Describe the specific instance used - graph structure, problem size, expected results]
//! - Instance: [e.g., "5-vertex graph from edges [(0,1), (1,2), ...]" or "Petersen graph"]
//! - Source: [expected optimal value]
//! - Target: [expected optimal value]
//! - Reference: [cite pkgref source if applicable, e.g., "Based on qubogen test case"]
//!
//! See docs/paper/reductions.typ for the full reduction specification.

use problemreductions::prelude::*;

fn main() {
    // 1. Create source problem
    // Use instances from reference packages where available:
    // - ProblemReductions.jl examples (Petersen graph, demo graphs)
    // - qubogen test cases (small graphs with known solutions)
    // - UnitDiskMapping.jl examples
    let source = SourceProblem::new(...);

    // 2. Reduce to target
    let reduction = ReduceTo::<TargetProblem>::reduce_to(&source);
    let target = reduction.target_problem();

    // 3. Print problem transformation metrics
    println!("\n=== Problem Transformation ===");
    println!("Source: {} with {} variables",
             SourceProblem::NAME, source.num_variables());
    println!("Target: {} with {} variables",
             TargetProblem::NAME, target.num_variables());

    // 4. Solve target problem
    let solver = BruteForce::new();
    let target_solutions = solver.find_best(target);
    println!("\n=== Solution ===");
    println!("Target solutions found: {}", target_solutions.len());

    // 5. Extract source solution
    let source_solution = reduction.extract_solution(&target_solutions[0]);
    println!("Source solution: {:?}", source_solution);

    // 6. Verify and print result
    let size = source.solution_size(&source_solution);
    println!("Solution size: {:?}", size);
    assert!(size.is_valid);
    println!("\n✓ Reduction verified successfully");
}
```

#### File Manifest (28 files)

**Note:** Unit Disk Mapping (IS → GridGraph IS) already has an example at `examples/export_petersen_mapping.rs`. We will link to it from the paper but not create a new example file.

**Trivial/Complement (6 files):**
1. `reduction_is_to_vc.rs`
2. `reduction_vc_to_is.rs`
3. `reduction_spinglass_to_qubo.rs`
4. `reduction_qubo_to_spinglass.rs`
5. `reduction_spinglass_to_maxcut.rs`
6. `reduction_maxcut_to_spinglass.rs`

**Graph → Set (3 files):**
7. `reduction_is_to_setpacking.rs`
8. `reduction_matching_to_setpacking.rs`
9. `reduction_vc_to_setcovering.rs`

**Penalty-method QUBO (6 files):**
10. `reduction_is_to_qubo.rs`
11. `reduction_vc_to_qubo.rs`
12. `reduction_coloring_to_qubo.rs`
13. `reduction_setpacking_to_qubo.rs`
14. `reduction_ksatisfiability_to_qubo.rs`
15. `reduction_ilp_to_qubo.rs`

**ILP formulations (9 files):**
16. `reduction_coloring_to_ilp.rs`
17. `reduction_factoring_to_ilp.rs`
18. `reduction_is_to_ilp.rs`
19. `reduction_vc_to_ilp.rs`
20. `reduction_matching_to_ilp.rs`
21. `reduction_setpacking_to_ilp.rs`
22. `reduction_setcovering_to_ilp.rs`
23. `reduction_dominatingset_to_ilp.rs`
24. `reduction_clique_to_ilp.rs`

**Non-trivial (6 files):**
25. `reduction_sat_to_is.rs`
26. `reduction_sat_to_coloring.rs`
27. `reduction_sat_to_dominatingset.rs`
28. `reduction_sat_to_ksat.rs`
29. `reduction_circuit_to_spinglass.rs`
30. `reduction_factoring_to_circuit.rs`

### 4. Reference Package Integration

Use instances from reference packages for cross-verification and consistency:

**Available in `pkgref/` (cloned from GitHub):**

1. **ProblemReductions.jl** (`pkgref/ProblemReductions.jl/examples/`)
   - Petersen graph examples
   - Factoring → Circuit → SpinGlass
   - Educational narrative style

2. **UnitDiskMapping.jl** (`pkgref/UnitDiskMapping.jl/examples/`)
   - 5-vertex demo graph: edges `[(1,2), (2,4), (3,4), (1,3), (4,5), (1,5)]`
   - Petersen graph mapping
   - Comprehensive tutorial examples

3. **qubogen** (`pkgref/qubogen/tests/`)
   - Small test instances (5 nodes) with known QUBO matrices
   - Graph coloring: 5 nodes, 3 colors
   - Max-Cut, MVC, Set Packing test cases
   - Max-2-SAT examples

**Instance Selection Strategy:**

- **Graph problems**: Use Petersen graph (10 vertices) or UnitDiskMapping's 5-vertex demo graph
- **QUBO reductions**: Cross-reference with qubogen test cases where applicable
- **SAT problems**: Small formulas (3-4 variables) with known solutions
- **Factoring**: Use 6 = 2×3 from ProblemReductions.jl example
- **Document reference source** in example docstring when using external instance

### 5. Implementation Workflow

**Pass 1: Add theorem labels**
- Scan all `#theorem[...]` blocks in Section 3
- Add `<thm:source-to-target>` labels
- Build mapping: reduction → label

**Pass 2: Enhance problem definitions**
- For each problem in Section 2:
  - Add field mapping paragraph after struct
  - Replace "Reduces to/from" with "Implemented reductions" + theorem refs

**Pass 3: Enhance theorem proofs**
- Expand trivial reduction proofs with variable mapping
- Add GitHub links after all theorems
  - For Unit Disk Mapping (IS → GridGraph IS): link to existing `examples/export_petersen_mapping.rs`
  - For other reductions: link to new `examples/reduction_*.rs` files

**Pass 4: Create example files**
- Extract embedded examples to standalone files
- For each new example:
  - Check `pkgref/` for matching instances in reference packages
  - Use reference instances where available for cross-verification
  - Document source in docstring (e.g., "Based on qubogen test case")
  - Add detailed output showing problem transformation metrics
- Each file: detailed docstring + closed-loop verification with metrics

**Pass 5: Verification**
- All theorems labeled
- All problems link to theorems
- All theorems link to examples
- Run `make paper` - must compile without errors

## Success Criteria

- [ ] 15 problem definitions have field mapping paragraphs
- [ ] All problem definitions link to theorem labels (not problem definitions)
- [ ] 28 theorems have labels and GitHub example links
- [ ] Trivial reduction proofs explain variable mappings explicitly
- [ ] 28 example files created with detailed docstrings
- [ ] All examples use Petersen-scale instances (non-trivial)
- [ ] `make paper` compiles successfully
- [ ] All example files compile and run successfully

## Dependencies

- Repository: https://github.com/CodingThrust/problem-reductions
- Paper file: `docs/paper/reductions.typ`
- Examples directory: `examples/` (already exists)
- Reduction rules: 28 files in `src/rules/`

## Notes

- **Docstrings**: Explain math and example instance, NOT reduction algorithm (kept in paper)
- **Instance selection**: Prefer instances from reference packages (ProblemReductions.jl, UnitDiskMapping.jl, qubogen) for cross-verification
- **Output style**: Inspired by UnitDiskMapping.jl - show problem transformation metrics and verification details
- **Separation of concerns**: Examples demonstrate mechanics, paper provides mathematical specification
- **GitHub links**: Use path `/blob/main/examples/reduction_*.rs`
- **Reference packages**: Located in `pkgref/` (gitignored, cloned for development reference)
