# Plan: Add HamiltonianCircuit Model (#216)

## Overview

Add the HamiltonianCircuit satisfaction problem — a classical NP-complete problem (Karp, 1972) asking whether an undirected graph contains a cycle visiting every vertex exactly once.

**Problem type:** SatisfactionProblem (Metric = bool)
**Category:** graph
**Type parameter:** G: Graph (no weight)

## Information Checklist

| # | Item | Value |
|---|------|-------|
| 1 | Problem name | `HamiltonianCircuit` |
| 2 | Definition | Given G=(V,E), does G contain a Hamiltonian circuit (closed path visiting every vertex exactly once)? |
| 3 | Problem type | Satisfaction (bool) |
| 4 | Type parameters | `G: Graph` |
| 5 | Struct fields | `graph: G` |
| 6 | Config space | `dims() = vec![n; n]` — permutation encoding: n positions, each picks a vertex (0..n-1) |
| 7 | Feasibility check | Config represents a valid permutation AND consecutive vertices (including wrap-around) are adjacent |
| 8 | Objective | None (satisfaction) — returns `true` if Hamiltonian circuit exists |
| 9 | Complexity | O(num_vertices^2 * 2^num_vertices) — Held-Karp DP (1962), deterministic |
| 10 | Solving strategy | BruteForce (enumerate all configs, check evaluate()) |
| 11 | Category | `graph` |

## Steps

### Step 1: Implement the model [INDEPENDENT]

Create `src/models/graph/hamiltonian_circuit.rs`:

1. **ProblemSchemaEntry** via `inventory::submit!`:
   - name: "HamiltonianCircuit"
   - description: "Does the graph contain a Hamiltonian circuit?"
   - fields: `[FieldInfo { name: "graph", type_name: "G", description: "The undirected graph G=(V,E)" }]`

2. **Struct definition:**
   ```rust
   #[derive(Debug, Clone, Serialize, Deserialize)]
   #[serde(bound(deserialize = "G: serde::Deserialize<'de>"))]
   pub struct HamiltonianCircuit<G> {
       graph: G,
   }
   ```

3. **Inherent methods:**
   - `pub fn new(graph: G) -> Self`
   - `pub fn graph(&self) -> &G`
   - `pub fn num_vertices(&self) -> usize` (delegates to graph)
   - `pub fn num_edges(&self) -> usize` (delegates to graph)

4. **Problem trait impl:**
   - `const NAME: &'static str = "HamiltonianCircuit"`
   - `type Metric = bool`
   - `fn variant()` → `crate::variant_params![G]`
   - `fn dims(&self)` → `vec![n; n]` where n = num_vertices (permutation encoding)
   - `fn evaluate(&self, config: &[usize])` → check:
     1. All values in 0..n (automatic from dims)
     2. Config is a valid permutation (no duplicates — use a boolean seen-array)
     3. For all i in 0..n: edge (config[i], config[(i+1) % n]) exists in graph
     Return true only if all checks pass.

5. **SatisfactionProblem impl:** marker trait, empty impl

6. **declare_variants!:**
   ```rust
   crate::declare_variants! {
       HamiltonianCircuit<SimpleGraph> => "num_vertices^2 * 2^num_vertices",
   }
   ```
   Uses Held-Karp complexity: O(n^2 * 2^n), deterministic.

7. **Test link:**
   ```rust
   #[cfg(test)]
   #[path = "../../unit_tests/models/graph/hamiltonian_circuit.rs"]
   mod tests;
   ```

### Step 2: Register the model [DEPENDS ON: Step 1]

1. **`src/models/graph/mod.rs`:** Add `pub(crate) mod hamiltonian_circuit;` and `pub use hamiltonian_circuit::HamiltonianCircuit;`
2. **`src/models/mod.rs`:** Add `HamiltonianCircuit` to the `pub use graph::{...}` line

### Step 3: Register in CLI [DEPENDS ON: Step 2]

1. **`problemreductions-cli/src/dispatch.rs`:**
   - In `load_problem()`: add match arm `"HamiltonianCircuit" => deser_sat::<HamiltonianCircuit<SimpleGraph>>(data)`
   - In `serialize_any_problem()`: add match arm `"HamiltonianCircuit" => try_ser::<HamiltonianCircuit<SimpleGraph>>(any)`

2. **`problemreductions-cli/src/problem_name.rs`:**
   - In `resolve_alias()`: add `"hamiltoniancircuit" => "HamiltonianCircuit".to_string()`
   - Add `("HC", "HamiltonianCircuit")` to `ALIASES` — HC is a well-established abbreviation in the literature

3. **`problemreductions-cli/src/commands/create.rs`:**
   - Add match arm for `"HamiltonianCircuit"` — parse `--graph` flag, construct `HamiltonianCircuit::new(graph)`, serialize
   - Pattern: similar to simple graph problems without weights

4. **`problemreductions-cli/src/cli.rs`:**
   - Add `HamiltonianCircuit` to the "Flags by problem type" table in after_help: `HamiltonianCircuit, HC         --graph`

### Step 4: Write unit tests [INDEPENDENT]

Create `src/unit_tests/models/graph/hamiltonian_circuit.rs`:

1. **`test_hamiltonian_circuit_basic`:**
   - Create prism graph (6 vertices, 9 edges from issue Example 1)
   - Verify dims() returns `vec![6; 6]`
   - Verify evaluate() returns true for valid Hamiltonian circuit `[0, 1, 2, 5, 4, 3]`
   - Verify evaluate() returns false for invalid configs (non-permutation, missing edge)

2. **`test_hamiltonian_circuit_no_solution`:**
   - Create graph from issue Example 2 (star + chords, no HC)
   - Use BruteForce solver to verify no satisfying solution exists

3. **`test_hamiltonian_circuit_solver`:**
   - Create a small graph (e.g., cycle on 4 vertices — square)
   - Use BruteForce to find all satisfying solutions
   - Verify each solution is a valid Hamiltonian circuit

4. **`test_hamiltonian_circuit_serialization`:**
   - Round-trip serde test: serialize → deserialize → verify equal evaluation

5. Register test file in `src/unit_tests/models/graph/mod.rs`

### Step 5: Document in paper [INDEPENDENT]

Update `docs/paper/reductions.typ`:

1. Add to `display-name` dict: `"HamiltonianCircuit": [Hamiltonian Circuit]`

2. Add `problem-def("HamiltonianCircuit")`:
   - **Definition:** Given G=(V,E), does G contain a Hamiltonian circuit — a closed path visiting every vertex exactly once?
   - **Body:** Background on NP-completeness (Karp 1972), relationship to TSP, Held-Karp algorithm, permutation encoding explanation. Include example with prism graph visualization.

### Step 6: Verify [DEPENDS ON: Steps 1-5]

```bash
make fmt
make clippy
make test
```

All must pass. Coverage >95% on new code.

## Parallelism

- Steps 1, 4, 5 are independent and can run in parallel
- Steps 2, 3 depend on Step 1
- Step 6 depends on all others
