# Reduce Exported Functions Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Clean up the public API surface by internalizing implementation details, consolidating validation functions into problem methods, and removing redundant modules.

**Architecture:** Five phases — (1) internalize reduction structs and implementation details in `src/rules/mod.rs`, (2) internalize support modules (`polynomial`, `truth_table`, `graph_types`), (3) add `is_valid_solution` methods and internalize free functions, (4) config consolidation, (5) update prelude. Each phase ends with `make test clippy`.

**Tech Stack:** Rust, `pub(crate)` visibility, inherent methods.

**Design doc:** `docs/plans/2026-02-16-reduce-exported-functions-design.md`

**Constraints discovered during planning:**
- `ReductionEntry`, `ReductionOverhead`, `ReductionAutoCast` MUST stay `pub` — referenced by `#[reduction]` proc macro and `impl_variant_reduction!` exported macro
- Integration tests (`tests/`) are a separate crate — `pub(crate)` items inaccessible there
- Validation functions take `&[bool]` while `Problem` trait uses `&[usize]` — new methods bridge this
- PR #76 added `problem_size()` free function and `problem_size_names()`/`problem_size_values()` to `Problem` trait — `problem_size` should stay in the prelude
- Some models already have private `_config` helpers (e.g., `is_independent_set_config`) — `is_valid_solution` methods should delegate to these where available

---

### Task 1: Internalize reduction structs in rules/mod.rs

**Files:**
- Modify: `src/rules/mod.rs`

**Step 1: Remove pub use for all individual ReductionXToY structs and gadgets**

In `src/rules/mod.rs`, replace all the `pub use` lines for reduction structs (lines 61-87 and 139-159) with `pub(crate) use` equivalents. Keep the trait re-exports and graph API exports.

The file should go from:

```rust
pub use circuit_spinglass::{
    and_gadget, not_gadget, or_gadget, set0_gadget, set1_gadget, xor_gadget, LogicGadget,
    ReductionCircuitToSG,
};
pub use coloring_qubo::ReductionKColoringToQUBO;
pub use factoring_circuit::ReductionFactoringToCircuit;
// ... all the other pub use lines for reduction structs ...
pub use sat_maximumindependentset::{BoolVar, ReductionSATToIS};
// ...
```

To:

```rust
pub(crate) use circuit_spinglass::{
    and_gadget, not_gadget, or_gadget, set0_gadget, set1_gadget, xor_gadget, LogicGadget,
    ReductionCircuitToSG,
};
pub(crate) use coloring_qubo::ReductionKColoringToQUBO;
pub(crate) use factoring_circuit::ReductionFactoringToCircuit;
// ... change ALL reduction struct pub use lines to pub(crate) use ...
```

Lines to change (all `pub use` → `pub(crate) use`):
- Lines 61-64: circuit_spinglass (gadgets + ReductionCircuitToSG)
- Line 65: coloring_qubo
- Line 66: factoring_circuit
- Lines 67-70: graph JSON types only (NodeJson, EdgeJson, ReductionGraphJson — keep ReductionGraph, ReductionPath, ExecutablePath, ChainedReduction, ReductionStep as pub)
- Line 71: ksatisfiability_qubo
- Line 72: maximumindependentset_gridgraph
- Line 73: maximumindependentset_maximumsetpacking
- Line 74: maximumindependentset_qubo
- Line 75: maximumindependentset_triangular
- Line 76: maximummatching_maximumsetpacking
- Line 77: maximumsetpacking_qubo
- Lines 78-79: minimumvertexcover_maximumindependentset, minimumvertexcover_minimumsetcovering
- Line 80: minimumvertexcover_qubo
- Line 81: sat_circuitsat
- Line 82: sat_coloring
- Line 83: sat_ksat
- Line 84: sat_maximumindependentset (BoolVar + ReductionSATToIS)
- Line 85: sat_minimumdominatingset
- Lines 86-87: spinglass_maxcut, spinglass_qubo
- Lines 139-159: all ILP reduction structs

**Keep as `pub use`** (do NOT change):
- Line 5: `pub use cost::{CustomCost, Minimize, MinimizeSteps, PathCostFn};`
- Line 6: `pub use registry::{ReductionEntry, ReductionOverhead};`
- Line 88: `pub use traits::{ReduceTo, ReductionAutoCast, ReductionResult};`

**Split the graph re-export** (line 67-70) into two:
```rust
pub use graph::{
    ChainedReduction, ExecutablePath, ReductionGraph, ReductionPath, ReductionStep,
};
pub(crate) use graph::{EdgeJson, NodeJson, ReductionGraphJson};
```

**Step 2: Run tests**

Run: `make test clippy`
Expected: PASS — all reduction structs are used internally via the trait system, not by name from external code.

**Step 3: Commit**

```bash
git add src/rules/mod.rs
git commit -m "refactor: internalize reduction structs and gadgets in rules module"
```

---

### Task 2: Internalize unitdiskmapping internals

**Files:**
- Modify: `src/rules/unitdiskmapping/mod.rs`

**Step 1: Change pub use to pub(crate) use for internal types**

Replace:
```rust
pub use copyline::{create_copylines, mis_overhead_copyline, remove_order, CopyLine};
pub use grid::{CellState, MappingGrid};
pub use pathdecomposition::{pathwidth, Layout, PathDecompositionMethod};
pub use traits::{apply_gadget, pattern_matches, unapply_gadget, Pattern, PatternCell};
pub use copyline::{copyline_weighted_locations_triangular, mis_overhead_copyline_triangular};
pub use weighted::{map_weights, trace_centers, Weightable};
```

With:
```rust
pub(crate) use copyline::{create_copylines, mis_overhead_copyline, remove_order, CopyLine};
pub(crate) use grid::{CellState, MappingGrid};
pub(crate) use pathdecomposition::{pathwidth, Layout, PathDecompositionMethod};
pub(crate) use traits::{apply_gadget, pattern_matches, unapply_gadget, Pattern, PatternCell};
pub(crate) use copyline::{copyline_weighted_locations_triangular, mis_overhead_copyline_triangular};
pub(crate) use weighted::{map_weights, trace_centers, Weightable};
```

Keep as `pub`:
```rust
pub mod ksg;
pub mod triangular;
pub use ksg::{GridKind, MappingResult};
```

Also change `pub mod alpha_tensor` and `pub mod pathdecomposition` to `pub(crate) mod` if they only contain internal types.

**Step 2: Run tests**

Run: `make test clippy`
Expected: PASS

**Step 3: Commit**

```bash
git add src/rules/unitdiskmapping/mod.rs
git commit -m "refactor: internalize unitdiskmapping implementation details"
```

---

### Task 3: Internalize polynomial, truth_table modules and delete graph_types

**Files:**
- Modify: `src/lib.rs`
- Delete: `src/graph_types.rs`

**Step 1: Change module visibility in lib.rs**

In `src/lib.rs`, change:
```rust
pub mod polynomial;
```
to:
```rust
pub(crate) mod polynomial;
```

Change:
```rust
pub mod truth_table;
```
to:
```rust
pub(crate) mod truth_table;
```

Change:
```rust
pub mod graph_types;
```
to: delete this line entirely. Then delete `src/graph_types.rs`.

Since `graph_types` has zero internal imports, no other files need updating.

**Step 2: Run tests**

Run: `make test clippy`
Expected: PASS — no code imports from these modules externally.

**Step 3: Commit**

```bash
git add src/lib.rs
git rm src/graph_types.rs
git commit -m "refactor: internalize polynomial/truth_table, delete unused graph_types module"
```

---

### Task 4: Add is_valid_solution methods to graph problems

**Files:**
- Modify: `src/models/graph/maximum_independent_set.rs`
- Modify: `src/models/graph/maximum_clique.rs`
- Modify: `src/models/graph/minimum_vertex_cover.rs`
- Modify: `src/models/graph/minimum_dominating_set.rs`
- Modify: `src/models/graph/maximum_matching.rs`
- Modify: `src/models/graph/kcoloring.rs`
- Modify: `src/models/graph/maximal_is.rs`
- Modify: `src/models/graph/traveling_salesman.rs`
- Modify: `src/models/graph/max_cut.rs`

**Step 1: Add is_valid_solution to each graph problem struct**

For each problem type, add an inherent method that converts `&[usize]` config to `&[bool]` and delegates to the existing validation function. Add these methods inside the existing `impl` block for each type.

**MaximumIndependentSet** (`src/models/graph/maximum_independent_set.rs`):
```rust
/// Check if a configuration is a valid independent set.
pub fn is_valid_solution(&self, config: &[usize]) -> bool {
    // Delegate to existing private helper that already takes &[usize]
    is_independent_set_config(self.graph(), config)
}
```
Note: This file already has a private `is_independent_set_config` helper taking `&[usize]`. Delegate to it directly instead of converting to `&[bool]`. Check other model files for similar `_config` helpers and use the same pattern.

**MaximumClique** (`src/models/graph/maximum_clique.rs`):
```rust
pub fn is_valid_solution(&self, config: &[usize]) -> bool {
    let selected: Vec<bool> = config.iter().map(|&v| v != 0).collect();
    is_clique(self.graph(), &selected)
}
```

**MinimumVertexCover** (`src/models/graph/minimum_vertex_cover.rs`):
```rust
pub fn is_valid_solution(&self, config: &[usize]) -> bool {
    let selected: Vec<bool> = config.iter().map(|&v| v != 0).collect();
    is_vertex_cover(self.graph(), &selected)
}
```

**MinimumDominatingSet** (`src/models/graph/minimum_dominating_set.rs`):
```rust
pub fn is_valid_solution(&self, config: &[usize]) -> bool {
    let selected: Vec<bool> = config.iter().map(|&v| v != 0).collect();
    is_dominating_set(self.graph(), &selected)
}
```

**MaximumMatching** (`src/models/graph/maximum_matching.rs`):
```rust
pub fn is_valid_solution(&self, config: &[usize]) -> bool {
    let selected: Vec<bool> = config.iter().map(|&v| v != 0).collect();
    is_matching(self.graph(), &selected)
}
```

**KColoring** (`src/models/graph/kcoloring.rs`):
```rust
pub fn is_valid_solution(&self, config: &[usize]) -> bool {
    is_valid_coloring(self.graph(), config, self.num_colors())
}
```
Note: `is_valid_coloring` already takes `&[usize]`, no conversion needed.

**MaximalIS** (`src/models/graph/maximal_is.rs`):
```rust
pub fn is_valid_solution(&self, config: &[usize]) -> bool {
    let selected: Vec<bool> = config.iter().map(|&v| v != 0).collect();
    is_maximal_independent_set(self.graph(), &selected)
}
```

**TravelingSalesman** (`src/models/graph/traveling_salesman.rs`):
```rust
pub fn is_valid_solution(&self, config: &[usize]) -> bool {
    let selected: Vec<bool> = config.iter().map(|&v| v != 0).collect();
    is_hamiltonian_cycle(self.graph(), &selected)
}
```

**MaxCut** (`src/models/graph/max_cut.rs`):
```rust
/// Compute the cut size for a given partition configuration.
pub fn cut_size(&self, config: &[usize]) -> <W as WeightElement>::Sum {
    let partition: Vec<bool> = config.iter().map(|&v| v != 0).collect();
    cut_size(self.graph(), self.weights(), &partition)
}
```

**Step 2: Run tests**

Run: `make test clippy`
Expected: PASS — new methods added, nothing removed yet.

**Step 3: Commit**

```bash
git add src/models/graph/
git commit -m "feat: add is_valid_solution methods to graph problem types"
```

---

### Task 5: Add is_valid_solution methods to set and specialized problems

**Files:**
- Modify: `src/models/set/maximum_set_packing.rs`
- Modify: `src/models/set/minimum_set_covering.rs`
- Modify: `src/models/specialized/biclique_cover.rs`
- Modify: `src/models/specialized/circuit.rs`
- Modify: `src/models/specialized/factoring.rs`
- Modify: `src/models/specialized/paintshop.rs`
- Modify: `src/models/specialized/bmf.rs`

**Step 1: Add methods**

**MaximumSetPacking** (`src/models/set/maximum_set_packing.rs`):
```rust
pub fn is_valid_solution(&self, config: &[usize]) -> bool {
    let selected: Vec<bool> = config.iter().map(|&v| v != 0).collect();
    is_set_packing(self.sets(), &selected)
}
```

**MinimumSetCovering** (`src/models/set/minimum_set_covering.rs`):
```rust
pub fn is_valid_solution(&self, config: &[usize]) -> bool {
    let selected: Vec<bool> = config.iter().map(|&v| v != 0).collect();
    is_set_cover(self.universe_size(), self.sets(), &selected)
}
```

**BicliqueCover** (`src/models/specialized/biclique_cover.rs`):
```rust
pub fn is_valid_solution(&self, config: &[usize]) -> bool {
    // Delegate to existing is_biclique_cover with problem's data
    // Implementation depends on how config maps to biclique selection
    // Check the existing evaluate() method for the mapping
    is_biclique_cover(self.edges(), self.left_bicliques(config), self.right_bicliques(config))
}
```
Note: Inspect the actual struct fields and `evaluate()` to determine correct delegation. The method body may need adjustment.

**CircuitSAT** (`src/models/specialized/circuit.rs`):
```rust
pub fn is_valid_solution(&self, config: &[usize]) -> bool {
    // Convert config to assignments HashMap and delegate
    // Check evaluate() for how config maps to variable assignments
    self.evaluate(config)
}
```
Note: For satisfaction problems where `Metric = bool`, `is_valid_solution` can just delegate to `evaluate`.

**Factoring** (`src/models/specialized/factoring.rs`):
```rust
pub fn is_valid_solution(&self, config: &[usize]) -> bool {
    self.evaluate(config)
}
```

**PaintShop** (`src/models/specialized/paintshop.rs`):
```rust
/// Count the number of paint switches for a given configuration.
pub fn count_switches(&self, config: &[usize]) -> usize {
    count_paint_switches(config)
}
```

**BMF** (`src/models/specialized/bmf.rs`) — no `is_valid_solution` needed (BMF is optimization). The matrix utility functions just become `pub(crate)`.

**Step 2: Run tests**

Run: `make test clippy`
Expected: PASS

**Step 3: Commit**

```bash
git add src/models/set/ src/models/specialized/
git commit -m "feat: add is_valid_solution/count_switches methods to set and specialized problems"
```

---

### Task 6: Internalize validation free functions

**Files:**
- Modify: `src/models/graph/mod.rs`
- Modify: `src/models/graph/maximum_independent_set.rs` (change `pub fn` to `pub(crate) fn`)
- Modify: `src/models/graph/maximum_clique.rs`
- Modify: `src/models/graph/minimum_vertex_cover.rs`
- Modify: `src/models/graph/minimum_dominating_set.rs`
- Modify: `src/models/graph/maximum_matching.rs`
- Modify: `src/models/graph/kcoloring.rs`
- Modify: `src/models/graph/maximal_is.rs`
- Modify: `src/models/graph/traveling_salesman.rs`
- Modify: `src/models/graph/max_cut.rs`
- Modify: `src/models/set/maximum_set_packing.rs`
- Modify: `src/models/set/minimum_set_covering.rs`
- Modify: `src/models/set/mod.rs`
- Modify: `src/models/specialized/mod.rs`
- Modify: `src/models/specialized/biclique_cover.rs`
- Modify: `src/models/specialized/bmf.rs`
- Modify: `src/models/specialized/circuit.rs`
- Modify: `src/models/specialized/factoring.rs`
- Modify: `src/models/specialized/paintshop.rs`
- Modify: `src/models/mod.rs`

**Step 1: Change function visibility**

In each model file, change the validation free function from `pub fn` to `pub(crate) fn`:
- `is_independent_set` → `pub(crate) fn is_independent_set`
- `is_clique` → `pub(crate) fn is_clique`
- `is_vertex_cover` → `pub(crate) fn is_vertex_cover`
- `is_dominating_set` → `pub(crate) fn is_dominating_set`
- `is_matching` → `pub(crate) fn is_matching`
- `is_valid_coloring` → `pub(crate) fn is_valid_coloring`
- `is_maximal_independent_set` → `pub(crate) fn is_maximal_independent_set`
- `is_hamiltonian_cycle` → `pub(crate) fn is_hamiltonian_cycle`
- `cut_size` → `pub(crate) fn cut_size`
- `is_set_packing` → `pub(crate) fn is_set_packing`
- `is_set_cover` → `pub(crate) fn is_set_cover`
- `is_biclique_cover` → `pub(crate) fn is_biclique_cover`
- `is_circuit_satisfying` → `pub(crate) fn is_circuit_satisfying`
- `is_factoring` → `pub(crate) fn is_factoring`
- `count_paint_switches` → `pub(crate) fn count_paint_switches`
- `boolean_matrix_product` → `pub(crate) fn boolean_matrix_product`
- `matrix_hamming_distance` → `pub(crate) fn matrix_hamming_distance`

**Step 2: Remove free functions from mod.rs re-exports**

In `src/models/graph/mod.rs`, change:
```rust
pub use kcoloring::{is_valid_coloring, KColoring};
pub use max_cut::{cut_size, MaxCut};
pub use maximal_is::{is_maximal_independent_set, MaximalIS};
pub use maximum_clique::{is_clique, MaximumClique};
pub use maximum_independent_set::{is_independent_set, MaximumIndependentSet};
pub use maximum_matching::{is_matching, MaximumMatching};
pub use minimum_dominating_set::{is_dominating_set, MinimumDominatingSet};
pub use minimum_vertex_cover::{is_vertex_cover, MinimumVertexCover};
pub use traveling_salesman::{is_hamiltonian_cycle, TravelingSalesman};
```
to:
```rust
pub use kcoloring::KColoring;
pub use max_cut::MaxCut;
pub use maximal_is::MaximalIS;
pub use maximum_clique::MaximumClique;
pub use maximum_independent_set::MaximumIndependentSet;
pub use maximum_matching::MaximumMatching;
pub use minimum_dominating_set::MinimumDominatingSet;
pub use minimum_vertex_cover::MinimumVertexCover;
pub use traveling_salesman::TravelingSalesman;
```

In `src/models/set/mod.rs`, remove validation functions from re-exports (keep only the problem types).

In `src/models/specialized/mod.rs`, change:
```rust
pub use biclique_cover::{is_biclique_cover, BicliqueCover};
pub use bmf::{boolean_matrix_product, matrix_hamming_distance, BMF};
pub use circuit::{is_circuit_satisfying, Assignment, BooleanExpr, BooleanOp, Circuit, CircuitSAT};
pub use factoring::{is_factoring, Factoring};
pub use paintshop::{count_paint_switches, PaintShop};
```
to:
```rust
pub use biclique_cover::BicliqueCover;
pub use bmf::BMF;
pub use circuit::{BooleanExpr, BooleanOp, Circuit, CircuitSAT};
pub use factoring::Factoring;
pub use paintshop::PaintShop;
```
Note: `Assignment` also becomes `pub(crate)`.

In `src/models/mod.rs`, remove re-exported validation functions — only keep problem type re-exports.

**Step 3: Update any integration test call sites**

Check `tests/suites/integration.rs` — the audit found it uses `problem.evaluate(sol)` not free functions, so no changes needed there.

Check `tests/suites/` for any other files using the free functions via `use problemreductions::models::graph::*` glob import. With `pub(crate)`, those functions simply won't be in scope — only the problem types will. This should be transparent unless tests call the functions by name.

**Step 4: Run tests**

Run: `make test clippy`
Expected: PASS — all internal unit tests use `pub(crate)` items fine. Integration tests don't call these functions directly.

**Step 5: Commit**

```bash
git add src/models/
git commit -m "refactor: internalize validation free functions, keep as problem methods"
```

---

### Task 7: Internalize config utility functions

**Files:**
- Modify: `src/config.rs`

**Step 1: Change visibility**

In `src/config.rs`, change:
```rust
pub fn config_to_bits(config: &[usize]) -> Vec<bool> {
```
to:
```rust
pub(crate) fn config_to_bits(config: &[usize]) -> Vec<bool> {
```

And:
```rust
pub fn bits_to_config(bits: &[bool]) -> Vec<usize> {
```
to:
```rust
pub(crate) fn bits_to_config(bits: &[bool]) -> Vec<usize> {
```

Keep `ConfigIterator`, `DimsIterator`, `index_to_config`, `config_to_index` as `pub`.

**Step 2: Run tests**

Run: `make test clippy`
Expected: PASS

**Step 3: Commit**

```bash
git add src/config.rs
git commit -m "refactor: internalize config_to_bits and bits_to_config"
```

---

### Task 8: Update prelude

**Files:**
- Modify: `src/lib.rs`

**Step 1: Slim down the prelude**

Replace the prelude in `src/lib.rs` with:

```rust
/// Prelude module for convenient imports.
pub mod prelude {
    // Problem types
    pub use crate::models::graph::{
        KColoring, MaxCut, MaximalIS, MaximumClique, MaximumIndependentSet, MaximumMatching,
        MinimumDominatingSet, MinimumVertexCover, TravelingSalesman,
    };
    pub use crate::models::optimization::{SpinGlass, QUBO};
    pub use crate::models::satisfiability::{CNFClause, KSatisfiability, Satisfiability};
    pub use crate::models::set::{MaximumSetPacking, MinimumSetCovering};
    pub use crate::models::specialized::{BicliqueCover, BMF, CircuitSAT, Factoring, PaintShop};

    // Core traits
    pub use crate::rules::{ReduceTo, ReductionResult};
    pub use crate::solvers::{BruteForce, Solver};
    pub use crate::traits::{problem_size, OptimizationProblem, Problem, SatisfactionProblem};

    // Types
    pub use crate::error::{ProblemError, Result};
    pub use crate::types::{Direction, One, ProblemSize, SolutionSize, Unweighted};
}
```

Items removed from prelude (still accessible via full path):
- `config::{bits_to_config, config_to_bits, config_to_index, index_to_config, ConfigIterator}`
- `registry::{ComplexityClass, ProblemInfo, ProblemMetadata}`
- `variant::{CastToParent, KValue, VariantParam, K1, K2, K3, K4, K5, KN}`
- `types::{NumericSize, WeightElement}`
- `models::optimization::{Comparison, LinearConstraint, ObjectiveSense, VarBounds, ILP}`

Items kept in prelude (from PR #76):
- `traits::problem_size` — core Problem API utility
- `types::ProblemSize` — return type of `problem_size()`

**Step 2: Fix compilation errors in examples and tests**

Many examples use `use problemreductions::prelude::*`. Items removed from the prelude may need explicit imports. Specifically:
- Examples that construct `ILP` instances need: `use problemreductions::models::optimization::{ILP, Comparison, LinearConstraint, ObjectiveSense, VarBounds};`
- Examples using `ReduceTo` — still in prelude, OK
- Examples using `ProblemSize` — add explicit import where needed
- Examples using `ComplexityClass` or `ProblemInfo` — add explicit import
- Examples using config functions — add `use problemreductions::config::*;`

Search for compilation errors and fix each by adding the specific import.

**Step 3: Run tests**

Run: `make test clippy`
Expected: PASS after fixing imports

**Step 4: Commit**

```bash
git add src/lib.rs examples/ tests/
git commit -m "refactor: slim down prelude to essential items"
```

---

### Task 9: Final verification

**Step 1: Full test suite**

Run: `make test clippy`
Expected: PASS

**Step 2: Check doc build**

Run: `cargo doc --no-deps 2>&1 | head -50`
Expected: No warnings about broken doc links

**Step 3: Verify the public API looks clean**

Run: `cargo doc --no-deps --open` and review the top-level documentation. Verify:
- No `ReductionXToY` structs visible in `rules` module docs
- No gadget functions visible
- No `NodeJson`/`EdgeJson` visible
- Problem types have `is_valid_solution` methods in their docs
- Prelude is clean and focused

**Step 4: Commit any doc fixes**

```bash
git add -A
git commit -m "docs: fix any doc link issues from API cleanup"
```
