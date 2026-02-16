# `problem_size()` Trait Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add `fn problem_size(&self) -> ProblemSize` to the `Problem` trait, implement it for all 21 problem types, validate overhead polynomial variable names in `find_cheapest_path`, and test.

**Architecture:** Add a required `problem_size(&self)` method to `Problem` trait. Each impl returns a `ProblemSize` with named components matching the overhead polynomial variable names used by `#[reduction(overhead = ...)]` annotations. Add `input_variable_names()` to `ReductionOverhead` to extract referenced variable names from polynomials. In `find_cheapest_path`, validate that overhead polynomial input variables are a subset of `source.problem_size()` component names — catching naming mismatches at path-finding time. Empty `input_size` skips validation for backward compatibility.

**Tech Stack:** Rust, no new dependencies.

---

### Task 1: Add `problem_size()` to `Problem` trait

**Files:**
- Modify: `src/traits.rs:7-25`

**Step 1: Add the method to the trait**

In `src/traits.rs`, add after the `variant()` method (line 24):

```rust
    /// Named size components for overhead estimation in reduction path-finding.
    fn problem_size(&self) -> crate::types::ProblemSize;
```

**Step 2: Verify it fails to compile (all 21 impls are now broken)**

Run: `cargo check 2>&1 | head -5`
Expected: errors about missing `problem_size` method in impls.

**Step 3: Commit**

```bash
git add src/traits.rs
git commit -m "feat: add problem_size() to Problem trait (compile-breaking)"
```

---

### Task 2: Implement `problem_size()` for graph problems (9 types)

All graph problems follow the same pattern: `num_vertices` from `self.graph().num_vertices()`, `num_edges` from `self.graph().num_edges()`. KColoring adds `num_colors`.

**Files:**
- Modify: `src/models/graph/maximum_independent_set.rs` (impl at ~line 93)
- Modify: `src/models/graph/maximum_clique.rs` (impl at ~line 93)
- Modify: `src/models/graph/minimum_vertex_cover.rs` (impl at ~line 88)
- Modify: `src/models/graph/minimum_dominating_set.rs` (impl at ~line 113)
- Modify: `src/models/graph/max_cut.rs` (impl at ~line 141)
- Modify: `src/models/graph/maximum_matching.rs` (impl at ~line 161)
- Modify: `src/models/graph/maximal_is.rs` (impl at ~line 127)
- Modify: `src/models/graph/kcoloring.rs` (impl at ~line 120)
- Modify: `src/models/graph/traveling_salesman.rs` (impl at ~line 124)

**Step 1: Add `use crate::types::ProblemSize;` to each file** (if not already imported).

**Step 2: Add `problem_size()` to each `impl Problem for` block**

For MIS, MaxClique, MinVC, MinDS, MaximalIS, MaxCut, MaximumMatching, TravelingSalesman:
```rust
    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("num_vertices", self.graph().num_vertices()),
            ("num_edges", self.graph().num_edges()),
        ])
    }
```

For KColoring (adds `num_colors`):
```rust
    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("num_vertices", self.graph().num_vertices()),
            ("num_edges", self.graph().num_edges()),
            ("num_colors", self.num_colors()),
        ])
    }
```

**Step 3: Verify graph models compile**

Run: `cargo check 2>&1 | grep "error\[" | head -5`
Expected: remaining errors only from non-graph models.

**Step 4: Commit**

```bash
git add src/models/graph/
git commit -m "feat: implement problem_size() for all graph problems"
```

---

### Task 3: Implement `problem_size()` for SAT problems (2 types)

**Files:**
- Modify: `src/models/satisfiability/sat.rs` (impl at ~line 171)
- Modify: `src/models/satisfiability/ksat.rs` (impl at ~line 161)

**Step 1: Add import and implement for Satisfiability**

```rust
    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("num_vars", self.num_vars()),
            ("num_clauses", self.num_clauses()),
            ("num_literals", self.num_literals()),
        ])
    }
```

**Step 2: Implement for KSatisfiability**

KSatisfiability does not have a `num_literals()` method. Compute inline:
```rust
    fn problem_size(&self) -> ProblemSize {
        let num_literals: usize = self.clauses().iter().map(|c| c.len()).sum();
        ProblemSize::new(vec![
            ("num_vars", self.num_vars()),
            ("num_clauses", self.num_clauses()),
            ("num_literals", num_literals),
        ])
    }
```

**Step 3: Commit**

```bash
git add src/models/satisfiability/
git commit -m "feat: implement problem_size() for SAT problems"
```

---

### Task 4: Implement `problem_size()` for set problems (2 types)

**Files:**
- Modify: `src/models/set/maximum_set_packing.rs` (impl at ~line 122)
- Modify: `src/models/set/minimum_set_covering.rs` (impl at ~line 132)

**Step 1: Implement for MaximumSetPacking**

MaximumSetPacking has `num_sets()` but no `universe_size()`. Compute from sets:
```rust
    fn problem_size(&self) -> ProblemSize {
        let universe_size = self.sets().iter()
            .flat_map(|s| s.iter())
            .max()
            .map_or(0, |&m| m + 1);
        ProblemSize::new(vec![
            ("num_sets", self.num_sets()),
            ("universe_size", universe_size),
        ])
    }
```

**Step 2: Implement for MinimumSetCovering**

```rust
    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("num_sets", self.num_sets()),
            ("universe_size", self.universe_size()),
        ])
    }
```

**Step 3: Commit**

```bash
git add src/models/set/
git commit -m "feat: implement problem_size() for set problems"
```

---

### Task 5: Implement `problem_size()` for optimization problems (3 types)

**Files:**
- Modify: `src/models/optimization/qubo.rs` (impl at ~line 146)
- Modify: `src/models/optimization/spin_glass.rs` (impl at ~line 198)
- Modify: `src/models/optimization/ilp.rs` (impl at ~line 330)

**Step 1: Implement for QUBO**

```rust
    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![("num_vars", self.num_vars())])
    }
```

**Step 2: Implement for SpinGlass**

SpinGlass has `num_spins()` (= `graph.num_vertices()`). For `num_interactions`, use `graph.num_edges()`:
```rust
    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("num_spins", self.num_spins()),
            ("num_interactions", self.graph().num_edges()),
        ])
    }
```

**Step 3: Implement for ILP**

ILP has `num_variables()` and `self.constraints` (pub field):
```rust
    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("num_vars", self.num_variables()),
            ("num_constraints", self.constraints.len()),
        ])
    }
```

**Step 4: Commit**

```bash
git add src/models/optimization/
git commit -m "feat: implement problem_size() for optimization problems"
```

---

### Task 6: Implement `problem_size()` for specialized problems (5 types)

**Files:**
- Modify: `src/models/specialized/factoring.rs` (impl at ~line 116)
- Modify: `src/models/specialized/circuit.rs` (impl at ~line 269)
- Modify: `src/models/specialized/paintshop.rs` (impl at ~line 163)
- Modify: `src/models/specialized/biclique_cover.rs` (impl at ~line 212)
- Modify: `src/models/specialized/bmf.rs` (impl at ~line 193)

**Step 1: Implement for Factoring**

```rust
    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("num_bits_first", self.m()),
            ("num_bits_second", self.n()),
        ])
    }
```

**Step 2: Implement for CircuitSAT**

CircuitSAT stores `variables: Vec<String>` and has `circuit.num_assignments()`:
```rust
    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("num_variables", self.num_variables()),
            ("num_assignments", self.circuit().num_assignments()),
        ])
    }
```

Note: verify that `self.circuit()` and `self.num_variables()` exist; if `circuit` is private without accessor, use `self.variables.len()` for `num_variables`.

**Step 3: Implement for PaintShop**

```rust
    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("num_cars", self.num_cars()),
            ("num_sequence", self.sequence_len()),
        ])
    }
```

**Step 4: Add accessors to BicliqueCover**

BicliqueCover has private `left_size` and `right_size` fields. Add public accessors:
```rust
    /// Get the left partition size.
    pub fn left_size(&self) -> usize {
        self.left_size
    }

    /// Get the right partition size.
    pub fn right_size(&self) -> usize {
        self.right_size
    }
```

Then implement:
```rust
    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("left_size", self.left_size()),
            ("right_size", self.right_size()),
            ("num_edges", self.num_edges()),
            ("rank", self.k()),
        ])
    }
```

**Step 5: Implement for BMF**

BMF has `rows()`, `cols()`, `rank()`:
```rust
    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("m", self.rows()),
            ("n", self.cols()),
            ("rank", self.rank()),
        ])
    }
```

**Step 6: Verify full project compiles**

Run: `cargo check --all-features`
Expected: clean compile, no errors.

**Step 7: Commit**

```bash
git add src/models/specialized/
git commit -m "feat: implement problem_size() for specialized problems"
```

---

### Task 7: Add overhead variable name validation in `find_cheapest_path`

**Files:**
- Modify: `src/polynomial.rs` (add `variable_names()` to `Polynomial`)
- Modify: `src/rules/registry.rs` (add `input_variable_names()` to `ReductionOverhead`)
- Modify: `src/rules/graph.rs` (add validation in `find_cheapest_path`)

**Step 1: Add `variable_names()` to `Polynomial`**

In `src/polynomial.rs`, add to `impl Polynomial`:
```rust
    /// Collect all variable names referenced by this polynomial.
    pub fn variable_names(&self) -> HashSet<&'static str> {
        self.terms.iter()
            .flat_map(|m| m.variables.iter().map(|(name, _)| *name))
            .collect()
    }
```

Add `use std::collections::HashSet;` at the top of the file.

**Step 2: Add `input_variable_names()` to `ReductionOverhead`**

In `src/rules/registry.rs`, add to `impl ReductionOverhead`:
```rust
    /// Collect all input variable names referenced by the overhead polynomials.
    pub fn input_variable_names(&self) -> HashSet<&'static str> {
        self.output_size.iter()
            .flat_map(|(_, poly)| poly.variable_names())
            .collect()
    }
```

Add `use std::collections::HashSet;` at the top.

**Step 3: Add validation in `find_cheapest_path`**

In `src/rules/graph.rs`, at the start of `find_cheapest_path`, after looking up the source node, validate that the `input_size` component names cover all overhead polynomial variables on outgoing edges. Skip validation when `input_size` is empty (backward compatibility):

```rust
    // Validate: when input_size is non-empty, check outgoing edges
    if !input_size.components.is_empty() {
        let size_names: std::collections::HashSet<&str> = input_size
            .components.iter().map(|(k, _)| k.as_str()).collect();
        for edge_ref in self.graph.edges(src) {
            let missing: Vec<_> = edge_ref.weight().overhead
                .input_variable_names()
                .into_iter()
                .filter(|name| !size_names.contains(name))
                .collect();
            if !missing.is_empty() {
                let target_node = &self.nodes[self.graph[edge_ref.target()]];
                panic!(
                    "Overhead for {} -> {} references variables {:?} \
                     not in source problem_size() components {:?}",
                    source, target_node.name, missing, size_names,
                );
            }
        }
    }
```

**Step 4: Verify compiles**

Run: `cargo check --all-features`

**Step 5: Commit**

```bash
git add src/polynomial.rs src/rules/registry.rs src/rules/graph.rs
git commit -m "feat: validate overhead variable names against problem_size in find_cheapest_path"
```

---

### Task 8: Add unit tests for `problem_size()`

**Files:**
- Create: `src/unit_tests/problem_size.rs`
- Modify: parent test module to include it via `#[path]` attribute

**Step 1: Write tests**

Create `src/unit_tests/problem_size.rs` with one test per problem category verifying component names and values match expectations. Example structure:

```rust
//! Tests for Problem::problem_size() implementations.

use crate::models::graph::*;
use crate::models::optimization::*;
use crate::models::satisfiability::*;
use crate::models::set::*;
use crate::models::specialized::*;
use crate::topology::SimpleGraph;
use crate::traits::Problem;

#[test]
fn test_problem_size_mis() {
    let g = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let mis = MaximumIndependentSet::<SimpleGraph, i32>::unweighted(g);
    let size = mis.problem_size();
    assert_eq!(size.get("num_vertices"), Some(4));
    assert_eq!(size.get("num_edges"), Some(3));
}

#[test]
fn test_problem_size_sat() {
    use crate::models::satisfiability::CNFClause;
    let sat = Satisfiability::new(3, vec![
        CNFClause::new(vec![1, -2]),
        CNFClause::new(vec![2, 3]),
    ]);
    let size = sat.problem_size();
    assert_eq!(size.get("num_vars"), Some(3));
    assert_eq!(size.get("num_clauses"), Some(2));
    assert_eq!(size.get("num_literals"), Some(4));
}

#[test]
fn test_problem_size_qubo() {
    let qubo = QUBO::<f64>::new(vec![1.0, 2.0, 3.0], vec![]);
    let size = qubo.problem_size();
    assert_eq!(size.get("num_vars"), Some(3));
}

#[test]
fn test_problem_size_spinglass() {
    let sg = SpinGlass::<SimpleGraph, f64>::new(
        3,
        vec![((0, 1), 1.0), ((1, 2), -1.0)],
        vec![0.0, 0.5, -0.5],
    );
    let size = sg.problem_size();
    assert_eq!(size.get("num_spins"), Some(3));
    assert_eq!(size.get("num_interactions"), Some(2));
}

#[test]
fn test_problem_size_factoring() {
    let f = Factoring::new(2, 3, 6);
    let size = f.problem_size();
    assert_eq!(size.get("num_bits_first"), Some(2));
    assert_eq!(size.get("num_bits_second"), Some(3));
}

#[test]
fn test_problem_size_bmf() {
    let bmf = BMF::new(vec![vec![true, false], vec![false, true]], 2);
    let size = bmf.problem_size();
    assert_eq!(size.get("m"), Some(2));
    assert_eq!(size.get("n"), Some(2));
    assert_eq!(size.get("rank"), Some(2));
}
```

Add similar tests for: MaxCut, KColoring, MaximumSetPacking, MinimumSetCovering, ILP, CircuitSAT, PaintShop, BicliqueCover. Each test creates a small instance and asserts the expected component names and values.

**Step 2: Wire the test module**

Find where unit test modules are declared (likely in `src/lib.rs` via `#[path]` attribute) and add:
```rust
#[cfg(test)]
#[path = "unit_tests/problem_size.rs"]
mod problem_size_tests;
```

**Step 3: Run tests**

Run: `cargo test --all-features problem_size`
Expected: all tests pass.

**Step 4: Commit**

```bash
git add src/unit_tests/problem_size.rs src/lib.rs
git commit -m "test: add problem_size() unit tests for all 21 problem types"
```

---

### Task 9: Add integration test for `find_cheapest_path` with `problem_size`

**Files:**
- Modify: `src/unit_tests/rules/reduction_path_parity.rs`

**Step 1: Add test using `find_cheapest_path` with real problem size**

```rust
#[test]
fn test_find_cheapest_path_with_problem_size() {
    let graph = ReductionGraph::new();
    let petersen = SimpleGraph::new(10, vec![
        (0,1),(0,4),(0,5),(1,2),(1,6),(2,3),(2,7),
        (3,4),(3,8),(4,9),(5,7),(5,8),(6,8),(6,9),(7,9),
    ]);
    let source = MaxCut::<SimpleGraph, i32>::unweighted(petersen);
    let src_var = ReductionGraph::variant_to_map(&MaxCut::<SimpleGraph, i32>::variant());
    let dst_var = ReductionGraph::variant_to_map(&SpinGlass::<SimpleGraph, f64>::variant());

    // Use source.problem_size() instead of ProblemSize::new(vec![])
    let rpath = graph
        .find_cheapest_path(
            "MaxCut", &src_var,
            "SpinGlass", &dst_var,
            &source.problem_size(),
            &MinimizeSteps,
        )
        .expect("Should find path MaxCut -> SpinGlass");

    assert!(!rpath.type_names().is_empty());

    // Verify problem_size has expected components
    let size = source.problem_size();
    assert_eq!(size.get("num_vertices"), Some(10));
    assert_eq!(size.get("num_edges"), Some(15));
}
```

**Step 2: Run test**

Run: `cargo test --all-features find_cheapest_path_with_problem_size`
Expected: pass (validation passes because MaxCut's components match overhead polynomial vars).

**Step 3: Commit**

```bash
git add src/unit_tests/
git commit -m "test: add find_cheapest_path integration test with problem_size"
```

---

### Task 10: Run full test suite and clippy

**Step 1: Run tests**

Run: `make test`
Expected: all tests pass.

**Step 2: Run clippy**

Run: `make clippy`
Expected: no warnings.

**Step 3: Run fmt**

Run: `make fmt`

**Step 4: Final commit if any formatting changes**

```bash
git add -A && git commit -m "chore: format"
```
