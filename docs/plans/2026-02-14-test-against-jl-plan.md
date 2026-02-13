# Test Against ProblemReductions.jl Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Full parity check between Rust `problemreductions` crate and Julia `ProblemReductions.jl` — reductions, evaluations, and solver results must match.

**Architecture:** Julia script generates JSON fixtures from ProblemReductions.jl test instances. Rust integration tests load those fixtures and verify parity. A Julia local environment in `scripts/jl/` keeps dependencies isolated.

**Tech Stack:** Julia (ProblemReductions.jl, Graphs.jl, JSON3.jl), Rust (serde_json, existing problemreductions crate)

---

### Task 1: Create Julia Local Environment

**Files:**
- Create: `scripts/jl/Project.toml`

**Step 1: Create Project.toml**

Create `scripts/jl/Project.toml`:

```toml
[deps]
ProblemReductions = "0cf46e72-2789-4cce-8199-8a1e3aff3551"
Graphs = "86223c79-3864-5bf0-83f7-82e725a168b6"
JSON = "682c06a0-de6a-54ab-a142-c8b1cf79cde6"
```

Note: The UUID for ProblemReductions must match the Julia General registry. Check `pkgref/ProblemReductions.jl/Project.toml` for the correct UUID.

**Step 2: Initialize and resolve**

```bash
cd scripts/jl && julia --project=. -e 'using Pkg; Pkg.instantiate()'
```

Expected: Downloads and precompiles packages. Generates `Manifest.toml`.

**Step 3: Verify it works**

```bash
cd scripts/jl && julia --project=. -e 'using ProblemReductions; println("OK: ", ProblemReductions)'
```

Expected: Prints `OK: ProblemReductions`

**Step 4: Commit**

```bash
git add scripts/jl/Project.toml scripts/jl/Manifest.toml
git commit -m "Add Julia local environment for ProblemReductions.jl testing"
```

---

### Task 2: Write Julia Test Data Generation Script

**Files:**
- Create: `scripts/jl/generate_testdata.jl`

This is the main Julia script. It constructs the same problem instances from `pkgref/ProblemReductions.jl/test/rules/rules.jl` (lines 56-91), runs evaluations/solvers/reductions, and exports JSON fixtures.

**Step 1: Write the script**

Create `scripts/jl/generate_testdata.jl`. The script must:

1. **Define helper functions:**
   - `graph_to_edges(g)` — convert Julia SimpleGraph to 0-based edge list `[[u,v], ...]`
   - `export_problem(problem)` — serialize a problem to a JSON-friendly dict with 0-based indices
   - `write_fixture(filename, data)` — write JSON to `../../tests/data/`

2. **Build test instances** (matching Julia test/rules/rules.jl):
   ```julia
   using ProblemReductions, Graphs, JSON

   graph = smallgraph(:petersen)
   circuit = CircuitSAT(@circuit begin
       x = a ∨ ¬b
       y = ¬c ∨ b
       z = x ∧ y ∧ a
   end)
   maxcut = MaxCut(graph)
   spinglass = SpinGlass(graph, [1,2,1,2,1,2,1,2,1,2,1,2,1,2,1], zeros(Int, nv(graph)))
   vertexcovering = VertexCovering(graph, [1,2,1,2,1,2,1,2,1,2])
   sat = Satisfiability(CNF([CNFClause([BoolVar(:a), BoolVar(:b)])]))
   ksat = KSatisfiability{3}(CNF([CNFClause([BoolVar(:a), BoolVar(:b), BoolVar(:c)])]))
   graph2 = HyperGraph(3, [[1, 2], [1], [2,3], [2]])
   qubo = QUBO([0 1 -2; 1 0 -2; -2 -2 6])
   is = IndependentSet(graph)
   is2 = IndependentSet(graph2)
   setpacking = SetPacking([[1, 2, 5], [1, 3], [2, 4], [3, 6], [2, 3, 6]])
   matching = Matching(graph)
   ```

3. **For each model**, generate evaluation fixtures:
   - Evaluate a handful of configs (including valid and invalid)
   - Run `findbest(problem, BruteForce())`
   - Export to `tests/data/jl_<problem>.json`

4. **For each reduction pair**, generate reduction fixtures:
   - Run `reduceto(TargetType, source)`
   - Export source problem, target problem (serialized with 0-based indices)
   - Run `findbest` on both
   - Run `extract_solution` and `extract_multiple_solutions`
   - Export to `tests/data/jl_<source>_to_<target>.json`

5. **Index conversion:** All vertex/variable indices must be converted from Julia 1-based to 0-based before writing JSON. This applies to:
   - Edge lists: subtract 1 from each vertex index
   - Solution configs: keep as-is (already 0/1 binary values)
   - SAT variable symbols: map to integer indices 0, 1, 2, ...
   - Set elements: subtract 1
   - HyperGraph hyperedges: subtract 1

**Key Julia → JSON mapping:**
- `SimpleGraph` → `{"num_vertices": N, "edges": [[u,v], ...]}`  (0-based)
- `HyperGraph` → `{"num_vertices": N, "hyperedges": [[v1,v2,...], ...]}`  (0-based)
- `IndependentSet` → `{"num_vertices": N, "edges": [[u,v],...], "weights": [...]}`
- `SpinGlass` → `{"num_vertices": N, "edges": [[u,v],...], "J": [...], "h": [...]}`
- `MaxCut` → `{"num_vertices": N, "edges": [[u,v],...], "weights": [...]}`
- `QUBO` → `{"matrix": [[...], ...]}`
- `Satisfiability` → `{"num_variables": N, "clauses": [{"literals": [{"variable": i, "negated": bool}, ...]}, ...]}`
- `KSatisfiability` → same as SAT plus `"k": K`
- `CircuitSAT` → `{"num_variables": N, "assignments": [...]}`  (complex; serialize expression tree)
- `VertexCovering` → `{"num_vertices": N, "edges": [[u,v],...], "weights": [...]}`
- `SetPacking` → `{"sets": [[e1,e2,...], ...], "weights": [...]}`
- `SetCovering` → `{"sets": [[e1,e2,...], ...], "weights": [...]}`
- `Matching` → `{"num_vertices": N, "edges": [[u,v],...], "weights": [...]}`
- `DominatingSet` → `{"num_vertices": N, "edges": [[u,v],...], "weights": [...]}`
- `Coloring{K}` → `{"num_vertices": N, "edges": [[u,v],...], "k": K, "weights": [...]}`
- `Factoring` → `{"m": M, "n": N, "input": T}`

**Reduction pairs to export** (from Julia test/rules/rules.jl lines 73-91):
```
circuit       => SpinGlass{SimpleGraph}
maxcut        => SpinGlass{SimpleGraph}
spinglass     => MaxCut
vertexcovering => SetCovering
sat           => Coloring{3}
qubo          => SpinGlass{SimpleGraph}
spinglass     => QUBO
sat           => KSatisfiability{3}
ksat          => Satisfiability
sat           => IndependentSet{SimpleGraph}
sat           => DominatingSet{SimpleGraph}
is            => SetPacking
is2           => SetPacking
setpacking    => IndependentSet{SimpleGraph}
is            => VertexCovering
matching      => SetPacking
```

Note: `Factoring => CircuitSAT` is tested separately in Julia (test/rules/factoring_sat.jl). Include it too.

**Step 2: Run the script**

```bash
cd scripts/jl && julia --project=. generate_testdata.jl
```

Expected: Creates ~20 JSON files in `tests/data/jl_*.json`.

**Step 3: Inspect outputs**

Verify a few JSON files look correct (0-based indices, proper structure).

**Step 4: Commit**

```bash
git add scripts/jl/generate_testdata.jl tests/data/jl_*.json
git commit -m "Add Julia test data generation script and fixtures"
```

---

### Task 3: Write Rust Parity Tests — Model Evaluations

**Files:**
- Create: `tests/suites/jl_parity.rs`
- Modify: `tests/main.rs` (add module)

**Step 1: Add module to tests/main.rs**

Add to `tests/main.rs`:
```rust
#[path = "suites/jl_parity.rs"]
mod jl_parity;
```

**Step 2: Write model evaluation tests**

Create `tests/suites/jl_parity.rs`. For each problem type that exists in both Rust and Julia, write a test that:

1. Loads `tests/data/jl_<problem>.json` via `include_str!`
2. Deserializes to `serde_json::Value`
3. Constructs the Rust problem from the JSON instance data
4. For each evaluation entry: calls `problem.evaluate(config)` and compares `size` and `is_valid`
5. Calls `BruteForce::find_all_best()` and compares solutions (as sets) with JSON `best_solutions`

**Problem name mapping** (Julia → Rust):
```rust
// Julia name             → Rust type
// "IndependentSet"       → MaximumIndependentSet<SimpleGraph, i32>
// "VertexCovering"       → MinimumVertexCover<SimpleGraph, i32>
// "MaxCut"               → MaxCut<SimpleGraph, i32>
// "SpinGlass"            → SpinGlass<SimpleGraph, i32> or f64
// "QUBO"                 → QUBO
// "Satisfiability"       → Satisfiability
// "KSatisfiability"      → KSatisfiability<K>
// "SetPacking"           → MaximumSetPacking<i32>
// "SetCovering"          → MinimumSetCovering<i32>
// "DominatingSet"        → MinimumDominatingSet<SimpleGraph, i32>
// "Matching"             → MaximumMatching<SimpleGraph, i32>
// "Coloring"             → KColoring<K, SimpleGraph>
// "CircuitSAT"           → CircuitSAT
// "Factoring"            → Factoring
```

**Test pattern:**
```rust
use problemreductions::prelude::*;
use problemreductions::topology::SimpleGraph;
use std::collections::HashSet;

#[test]
fn test_jl_parity_independentset_evaluation() {
    let data: serde_json::Value = serde_json::from_str(
        include_str!("../data/jl_independentset.json")
    ).unwrap();

    for instance in data["instances"].as_array().unwrap() {
        let num_vertices = instance["instance"]["num_vertices"].as_u64().unwrap() as usize;
        let edges: Vec<(usize, usize)> = instance["instance"]["edges"].as_array().unwrap()
            .iter().map(|e| {
                let arr = e.as_array().unwrap();
                (arr[0].as_u64().unwrap() as usize, arr[1].as_u64().unwrap() as usize)
            }).collect();
        let weights: Vec<i32> = instance["weights"].as_array().unwrap()
            .iter().map(|w| w.as_i64().unwrap() as i32).collect();

        let problem = if weights.iter().all(|&w| w == 1) {
            MaximumIndependentSet::<SimpleGraph, i32>::new(num_vertices, edges)
        } else {
            MaximumIndependentSet::with_weights(num_vertices, edges, weights)
        };

        // Check evaluations
        for eval in instance["evaluations"].as_array().unwrap() {
            let config: Vec<usize> = eval["config"].as_array().unwrap()
                .iter().map(|v| v.as_u64().unwrap() as usize).collect();
            let result = problem.evaluate(&config);
            let jl_valid = eval["is_valid"].as_bool().unwrap();
            assert_eq!(result.is_valid(), jl_valid);
            if jl_valid {
                let jl_size = eval["size"].as_i64().unwrap() as i32;
                assert_eq!(result.value(), jl_size);
            }
        }

        // Check best solutions
        let solver = BruteForce::new();
        let best = solver.find_all_best(&problem);
        let jl_best: HashSet<Vec<usize>> = instance["best_solutions"].as_array().unwrap()
            .iter().map(|s| s.as_array().unwrap().iter()
                .map(|v| v.as_u64().unwrap() as usize).collect()
            ).collect();
        let rust_best: HashSet<Vec<usize>> = best.into_iter().collect();
        assert_eq!(rust_best, jl_best, "Best solutions mismatch");
    }
}
```

Write similar tests for: `spinglass`, `maxcut`, `qubo`, `satisfiability`, `setpacking`, `vertexcovering`, `matching`, `factoring`. For SAT-type problems, compare `find_all_satisfying` instead of `find_all_best`.

**Step 3: Run tests**

```bash
cd /path/to/worktree && cargo test --test main jl_parity -- --nocapture
```

Expected: All evaluation tests pass.

**Step 4: Commit**

```bash
git add tests/suites/jl_parity.rs tests/main.rs
git commit -m "Add Rust parity tests for model evaluations against Julia"
```

---

### Task 4: Write Rust Parity Tests — Reductions

**Files:**
- Modify: `tests/suites/jl_parity.rs`

**Step 1: Write reduction parity tests**

For each reduction fixture `tests/data/jl_<source>_to_<target>.json`, add a test:

```rust
#[test]
fn test_jl_parity_independentset_to_vertexcovering() {
    let data: serde_json::Value = serde_json::from_str(
        include_str!("../data/jl_independentset_to_vertexcovering.json")
    ).unwrap();

    for case in data["cases"].as_array().unwrap() {
        // Construct source problem from JSON
        let source = /* deserialize from case["source"] */;

        // Reduce
        let result = ReduceTo::<MinimumVertexCover<SimpleGraph, i32>>::reduce_to(&source);
        let target = result.target_problem();

        // Solve both
        let solver = BruteForce::new();
        let best_target = solver.find_all_best(target);
        let best_source = solver.find_all_best(&source);

        // Extract solutions
        let extracted: HashSet<Vec<usize>> = best_target.iter()
            .map(|t| result.extract_solution(t))
            .collect();

        // Verify: extracted solutions should be a subset of best source solutions
        let best_source_set: HashSet<Vec<usize>> = best_source.into_iter().collect();
        assert!(extracted.is_subset(&best_source_set),
            "Extracted solutions should be among best source solutions");

        // Compare with Julia's extracted solutions
        let jl_extracted: HashSet<Vec<usize>> = case["extracted_solutions"].as_array().unwrap()
            .iter().map(|s| /* parse */).collect();

        // The Rust extracted set should match Julia's
        // (may differ in ordering but the SET should match)
        assert_eq!(extracted, jl_extracted);
    }
}
```

**Reductions with Rust implementations** (write active tests):
- `independentset_to_setpacking`
- `setpacking_to_independentset`
- `independentset_to_vertexcovering`
- `vertexcovering_to_setcovering`
- `spinglass_to_maxcut`
- `maxcut_to_spinglass`
- `spinglass_to_qubo`
- `qubo_to_spinglass`
- `sat_to_ksat`
- `ksat_to_sat`
- `circuitsat_to_spinglass`
- `factoring_to_circuitsat`

**Reductions WITHOUT Rust implementations** (write `#[ignore]` stubs):
- `sat_to_coloring`
- `sat_to_independentset`
- `sat_to_dominatingset`
- `matching_to_setpacking`

```rust
#[test]
#[ignore] // Not yet implemented in Rust
fn test_jl_parity_sat_to_coloring() {
    // TODO: Implement SAT → Coloring{3} reduction in Rust
}
```

**Step 2: Run tests**

```bash
cd /path/to/worktree && cargo test --test main jl_parity -- --nocapture
```

Expected: Active tests pass, ignored tests show as `ignored`.

**Step 3: Commit**

```bash
git add tests/suites/jl_parity.rs
git commit -m "Add Rust parity tests for reductions against Julia"
```

---

### Task 5: Verify and Clean Up

**Step 1: Run full test suite**

```bash
make test clippy
```

Expected: All tests pass, no clippy warnings.

**Step 2: Verify ignored tests list**

```bash
cargo test --test main jl_parity -- --ignored --list
```

Expected: Shows 4 ignored tests (sat_to_coloring, sat_to_independentset, sat_to_dominatingset, matching_to_setpacking).

**Step 3: Add Makefile target (optional)**

If desired, add to Makefile:
```makefile
jl-testdata:  ## Regenerate Julia parity test data
	cd scripts/jl && julia --project=. generate_testdata.jl
```

**Step 4: Final commit**

```bash
git add -A
git commit -m "Final cleanup for Julia parity testing"
```

---

### Task 6: Create PR

Create a pull request with:
- Title: `Test against ProblemReductions.jl (#64)`
- Summary of what was added (Julia env, script, fixtures, Rust tests)
- List of ignored tests as known gaps
