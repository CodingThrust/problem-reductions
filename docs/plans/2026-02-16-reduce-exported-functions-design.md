# Design: Reduce Exported Functions (#77)

## Goal

Ensure every exported item has clear meaning, is well-documented, and relates to the package's vision of NP-hard problem reductions. Items that are implementation details become `pub(crate)`.

## Approach: Moderate (Approach B)

v0.1.x allows breaking changes — items are simply made `pub(crate)` with no deprecation.

---

## 1. Internalize Reduction Structs & Implementation Details

### 1a. Reduction structs (`src/rules/mod.rs`)

Remove all `pub use` for individual `ReductionXToY` structs. Users interact via the `ReduceTo` trait or `ReductionGraph` — they never need the named struct types.

**Items to internalize (~30+):**
- `ReductionCircuitToSG`, `ReductionKColoringToQUBO`, `ReductionFactoringToCircuit`
- `Reduction3SATToQUBO`, `ReductionKSatToQUBO`
- `ReductionISSimpleToGrid`, `ReductionISUnitDiskToGrid`
- `ReductionISToSP`, `ReductionSPToIS`, `ReductionISToQUBO`
- `ReductionISSimpleToTriangular`
- `ReductionMatchingToSP`, `ReductionSPToQUBO`
- `ReductionVCToIS`, `ReductionISToVC`, `ReductionVCToSC`, `ReductionVCToQUBO`
- `ReductionSATToCircuit`, `ReductionSATToColoring`
- `ReductionKSATToSAT`, `ReductionSATToKSAT`
- `ReductionSATToIS`, `ReductionSATToDS`
- `ReductionMaxCutToSG`, `ReductionSGToMaxCut`
- `ReductionQUBOToSG`, `ReductionSGToQUBO`
- All ILP reduction structs (feature-gated)

### 1b. Circuit gadgets

Internalize: `and_gadget`, `or_gadget`, `not_gadget`, `xor_gadget`, `set0_gadget`, `set1_gadget`, `LogicGadget`

### 1c. Helper types

Internalize: `BoolVar` (from `sat_maximumindependentset`), `ReductionAutoCast` struct

### 1d. Unitdiskmapping internals (`src/rules/unitdiskmapping/mod.rs`)

Internalize: `CopyLine`, `CellState`, `MappingGrid`, `Pattern`, `PatternCell`, `apply_gadget`, `pattern_matches`, `unapply_gadget`, `Weightable`, `map_weights`, `trace_centers`, `create_copylines`, `remove_order`, `mis_overhead_copyline`, `copyline_weighted_locations_triangular`, `mis_overhead_copyline_triangular`

Keep public: `ksg`, `triangular` submodules, `MappingResult`, `GridKind`

### 1e. Other internals

- `src/polynomial.rs` -> `pub(crate) mod polynomial` in lib.rs
- `src/truth_table.rs` -> `pub(crate) mod truth_table` in lib.rs
- `NodeJson`, `EdgeJson`, `ReductionGraphJson` -> `pub(crate)` in `src/rules/graph.rs`
- `ReductionEntry`, `ReductionOverhead` -> `pub(crate)` in `src/rules/registry.rs` (already internal to proc macro + graph)

**Keep public in `src/rules/mod.rs`:**
- Traits: `ReduceTo`, `ReductionResult`
- Graph API: `ReductionGraph`, `ReductionPath`, `ExecutablePath`, `ChainedReduction`, `ReductionStep`
- Cost: `CustomCost`, `Minimize`, `MinimizeSteps`, `PathCostFn`

---

## 2. Consolidate Validation Functions into Problem Methods

Add `is_valid_solution(&self, config: &[usize]) -> bool` inherent methods to each problem type. The existing free functions become `pub(crate)`.

| Free function | Problem type | New method |
|---|---|---|
| `is_independent_set` | `MaximumIndependentSet` | `is_valid_solution` |
| `is_clique` | `MaximumClique` | `is_valid_solution` |
| `is_vertex_cover` | `MinimumVertexCover` | `is_valid_solution` |
| `is_dominating_set` | `MinimumDominatingSet` | `is_valid_solution` |
| `is_matching` | `MaximumMatching` | `is_valid_solution` |
| `is_valid_coloring` | `KColoring` | `is_valid_solution` |
| `is_maximal_independent_set` | `MaximalIS` | `is_valid_solution` |
| `is_hamiltonian_cycle` | `TravelingSalesman` | `is_valid_solution` |
| `is_satisfying_assignment` | `Satisfiability` | `is_valid_solution` |
| `is_biclique_cover` | `BicliqueCover` | `is_valid_solution` |
| `is_circuit_satisfying` | `CircuitSAT` | `is_valid_solution` |
| `is_factoring` | `Factoring` | `is_valid_solution` |
| `is_set_packing` | `MaximumSetPacking` | `is_valid_solution` |
| `is_set_cover` | `MinimumSetCovering` | `is_valid_solution` |

Value-returning functions also become methods:

| Free function | Problem type | New method |
|---|---|---|
| `cut_size` | `MaxCut` | `cut_size(&self, config)` |
| `count_paint_switches` | `PaintShop` | `count_switches(&self, config)` |

General matrix utilities become `pub(crate)`:
- `boolean_matrix_product` — not owned by BMF type, internal utility
- `matrix_hamming_distance` — same

---

## 3. Config & Module Consolidation

### 3a. Config utilities (`src/config.rs`)

Make `pub(crate)`: `config_to_bits`, `bits_to_config`

Keep public: `ConfigIterator`, `DimsIterator`, `index_to_config`, `config_to_index`

### 3b. Remove `graph_types` module

`src/graph_types.rs` is redundant with `src/topology/`. Delete and update all internal `use crate::graph_types::X` to `use crate::topology::X`.

### 3c. Circuit support types

- Keep public: `Circuit`, `BooleanExpr`, `BooleanOp` (needed to construct `CircuitSAT`)
- Make `pub(crate)`: `Assignment` (internal evaluation detail)

---

## 4. Updated Prelude

```rust
pub mod prelude {
    // Problem types
    pub use crate::models::graph::{
        KColoring, MaxCut, MaximalIS, MaximumClique, MaximumIndependentSet,
        MaximumMatching, MinimumDominatingSet, MinimumVertexCover, TravelingSalesman,
    };
    pub use crate::models::optimization::{SpinGlass, QUBO};
    pub use crate::models::satisfiability::{CNFClause, KSatisfiability, Satisfiability};
    pub use crate::models::set::{MaximumSetPacking, MinimumSetCovering};
    pub use crate::models::specialized::{BicliqueCover, BMF, CircuitSAT, Factoring, PaintShop};

    // Core traits
    pub use crate::traits::{OptimizationProblem, Problem, SatisfactionProblem};
    pub use crate::rules::{ReduceTo, ReductionResult};
    pub use crate::solvers::{BruteForce, Solver};

    // Types
    pub use crate::types::{Direction, One, SolutionSize, Unweighted};
    pub use crate::error::{ProblemError, Result};
}
```

**Removed from prelude** (still accessible via full path):
- `config::*` — power-user utilities
- `registry::{ComplexityClass, ProblemInfo, ProblemMetadata}` — metadata introspection
- `variant::{CastToParent, KValue, VariantParam, K1..KN}` — type-level machinery
- `types::{NumericSize, ProblemSize, WeightElement}` — trait bounds users rarely write

---

## 5. Final Public Module Summary

| Module | Visibility | Key public items |
|--------|-----------|-----------------|
| `models` | `pub` | All problem types, `Circuit`, `BooleanExpr`, `BooleanOp` |
| `rules` | `pub` | `ReduceTo`, `ReductionResult`, `ReductionGraph`, path types, cost types, `unitdiskmapping::{ksg, triangular, MappingResult, GridKind}` |
| `solvers` | `pub` | `Solver`, `BruteForce`, `ILPSolver` (feature-gated) |
| `traits` | `pub` | `Problem`, `OptimizationProblem`, `SatisfactionProblem` |
| `types` | `pub` | `Direction`, `SolutionSize`, `One`/`Unweighted`, `NumericSize`, `WeightElement`, `ProblemSize` |
| `error` | `pub` | `ProblemError`, `Result` |
| `config` | `pub` | `ConfigIterator`, `DimsIterator`, `index_to_config`, `config_to_index` |
| `topology` | `pub` | Graph types, `Graph` trait, `small_graphs` |
| `export` | `pub` | JSON export types (used by examples) |
| `io` | `pub` | `read_problem`, `write_problem`, `to_json`, `from_json` |
| `registry` | `pub` | `ProblemInfo`, `ComplexityClass`, `ProblemMetadata`, `collect_schemas` |
| `variant` | `pub` | `VariantParam`, `CastToParent`, `KValue`, K markers |
| `testing` | `pub` | Test macros & helpers |
| `polynomial` | `pub(crate)` | Internal |
| `truth_table` | `pub(crate)` | Internal |
| `graph_types` | **deleted** | Consolidated into `topology` |
