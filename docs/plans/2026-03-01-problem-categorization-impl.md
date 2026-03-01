# Problem Categorization Refactoring Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Move problem model files from the old mixed-axis categories (`optimization/`, `satisfiability/`, `specialized/`) to new input-structure-based categories (`formula/`, `algebraic/`, `misc/`), keeping `graph/` and `set/` unchanged.

**Architecture:** Pure file-move refactoring. The `classify_problem_category()` function in `src/rules/graph.rs` automatically derives categories from `module_path!()`, so moving files is sufficient — no classification logic changes. The `graph/` and `set/` modules are untouched. The `satisfiability/` module is renamed to `formula/` (with CircuitSAT added). The `optimization/` and `specialized/` modules are eliminated.

**Tech Stack:** Rust, `git mv`, `make check`

**Key reference:** Design doc at `docs/plans/2026-03-01-problem-categorization-design.md`

---

### Task 1: Create new module directories and move source files

**Files:**
- Create: `src/models/formula/mod.rs`, `src/models/algebraic/mod.rs`, `src/models/misc/mod.rs`
- Move (git mv): 13 source files between directories
- Delete: `src/models/optimization/`, `src/models/satisfiability/`, `src/models/specialized/` (after moves)

**Step 1: Move satisfiability/ → formula/ (rename + add CircuitSAT)**

```bash
# Rename the directory
git mv src/models/satisfiability src/models/formula

# Move CircuitSAT from specialized/ to formula/
git mv src/models/specialized/circuit.rs src/models/formula/circuit.rs
```

**Step 2: Create algebraic/ and move files into it**

```bash
mkdir -p src/models/algebraic

git mv src/models/optimization/qubo.rs src/models/algebraic/qubo.rs
git mv src/models/optimization/ilp.rs src/models/algebraic/ilp.rs
git mv src/models/optimization/closest_vector_problem.rs src/models/algebraic/closest_vector_problem.rs
git mv src/models/specialized/bmf.rs src/models/algebraic/bmf.rs
```

**Step 3: Create misc/ and move files into it**

```bash
mkdir -p src/models/misc

git mv src/models/optimization/bin_packing.rs src/models/misc/bin_packing.rs
git mv src/models/specialized/factoring.rs src/models/misc/factoring.rs
git mv src/models/specialized/paintshop.rs src/models/misc/paintshop.rs
```

**Step 4: Move SpinGlass and BicliqueCover into graph/**

```bash
git mv src/models/optimization/spin_glass.rs src/models/graph/spin_glass.rs
git mv src/models/specialized/biclique_cover.rs src/models/graph/biclique_cover.rs
```

**Step 5: Remove now-empty old directories**

```bash
# optimization/ and specialized/ should be empty except for mod.rs
rm src/models/optimization/mod.rs
rmdir src/models/optimization
rm src/models/specialized/mod.rs
rmdir src/models/specialized
```

**Step 6: Commit**

```bash
git add -A src/models/
git commit -m "refactor: move model files to input-structure-based categories"
```

---

### Task 2: Update mod.rs files for new module structure

**Files:**
- Modify: `src/models/mod.rs`
- Modify: `src/models/formula/mod.rs` (was satisfiability/mod.rs)
- Create: `src/models/algebraic/mod.rs`
- Create: `src/models/misc/mod.rs`
- Modify: `src/models/graph/mod.rs`

**Step 1: Update `src/models/mod.rs`**

Replace entire contents with:

```rust
//! Problem model implementations.
//!
//! Each sub-module groups related problem types by input structure.

pub mod algebraic;
pub mod formula;
pub mod graph;
pub mod misc;
pub mod set;

// Re-export commonly used types
pub use algebraic::{BinPacking, ClosestVectorProblem, SpinGlass, ILP, QUBO};
pub use formula::{CNFClause, KSatisfiability, Satisfiability};
pub use graph::{
    KColoring, MaxCut, MaximalIS, MaximumClique, MaximumIndependentSet, MaximumMatching,
    MinimumDominatingSet, MinimumVertexCover, TravelingSalesman,
};
pub use set::{MaximumSetPacking, MinimumSetCovering};
pub use specialized::{BicliqueCover, CircuitSAT, Factoring, PaintShop, BMF};
```

Wait — re-exports need to come from the *new* modules. Correct version:

```rust
//! Problem model implementations.
//!
//! Each sub-module groups related problem types by input structure.

pub mod algebraic;
pub mod formula;
pub mod graph;
pub mod misc;
pub mod set;

// Re-export commonly used types
pub use algebraic::{ClosestVectorProblem, ILP, QUBO, BMF};
pub use formula::{CNFClause, CircuitSAT, KSatisfiability, Satisfiability};
pub use graph::{
    BicliqueCover, KColoring, MaxCut, MaximalIS, MaximumClique, MaximumIndependentSet,
    MaximumMatching, MinimumDominatingSet, MinimumVertexCover, SpinGlass, TravelingSalesman,
};
pub use misc::{BinPacking, Factoring, PaintShop};
pub use set::{MaximumSetPacking, MinimumSetCovering};
```

**Step 2: Update `src/models/formula/mod.rs` (was satisfiability/mod.rs)**

Replace entire contents with:

```rust
//! Logic and formula problems.
//!
//! Problems whose input is a boolean formula or circuit:
//! - [`Satisfiability`]: Boolean satisfiability (SAT) with CNF clauses
//! - [`KSatisfiability`]: K-SAT where each clause has exactly K literals
//! - [`CircuitSAT`]: Boolean circuit satisfiability

mod ksat;
mod sat;
pub(crate) mod circuit;

pub use circuit::{Assignment, BooleanExpr, BooleanOp, Circuit, CircuitSAT};
pub use ksat::KSatisfiability;
pub use sat::{CNFClause, Satisfiability};
```

**Step 3: Write `src/models/algebraic/mod.rs`**

```rust
//! Algebraic problems.
//!
//! Problems whose input is a matrix, linear system, or lattice:
//! - [`QUBO`]: Quadratic Unconstrained Binary Optimization
//! - [`ILP`]: Integer Linear Programming
//! - [`ClosestVectorProblem`]: Closest Vector Problem (minimize lattice distance)
//! - [`BMF`]: Boolean Matrix Factorization

pub(crate) mod bmf;
mod closest_vector_problem;
mod ilp;
mod qubo;

pub use bmf::BMF;
pub use closest_vector_problem::ClosestVectorProblem;
pub use ilp::{Comparison, LinearConstraint, ObjectiveSense, VarBounds, ILP};
pub use qubo::QUBO;
```

**Step 4: Write `src/models/misc/mod.rs`**

```rust
//! Miscellaneous problems.
//!
//! Problems with unique input structures that don't fit other categories:
//! - [`BinPacking`]: Bin Packing (minimize bins)
//! - [`Factoring`]: Integer factorization
//! - [`PaintShop`]: Minimize color switches in paint shop scheduling

mod bin_packing;
pub(crate) mod factoring;
pub(crate) mod paintshop;

pub use bin_packing::BinPacking;
pub use factoring::Factoring;
pub use paintshop::PaintShop;
```

**Step 5: Update `src/models/graph/mod.rs`** — add SpinGlass and BicliqueCover

Add to the module declarations (maintaining alphabetical order and `pub(crate)` visibility where the old modules used it):

```rust
//! Graph problems.
//!
//! Problems whose input is a graph (optionally weighted):
//! - [`MaximumIndependentSet`]: Maximum weight independent set
//! - [`MaximalIS`]: Maximal independent set
//! - [`MinimumVertexCover`]: Minimum weight vertex cover
//! - [`MinimumDominatingSet`]: Minimum dominating set
//! - [`MaximumClique`]: Maximum weight clique
//! - [`MaxCut`]: Maximum cut on weighted graphs
//! - [`KColoring`]: K-vertex coloring
//! - [`MaximumMatching`]: Maximum weight matching
//! - [`TravelingSalesman`]: Traveling Salesman (minimum weight Hamiltonian cycle)
//! - [`SpinGlass`]: Ising model Hamiltonian
//! - [`BicliqueCover`]: Biclique cover on bipartite graphs

pub(crate) mod biclique_cover;
pub(crate) mod kcoloring;
pub(crate) mod max_cut;
pub(crate) mod maximal_is;
pub(crate) mod maximum_clique;
pub(crate) mod maximum_independent_set;
pub(crate) mod maximum_matching;
pub(crate) mod minimum_dominating_set;
pub(crate) mod minimum_vertex_cover;
pub(crate) mod spin_glass;
pub(crate) mod traveling_salesman;

pub use biclique_cover::BicliqueCover;
pub use kcoloring::KColoring;
pub use max_cut::MaxCut;
pub use maximal_is::MaximalIS;
pub use maximum_clique::MaximumClique;
pub use maximum_independent_set::MaximumIndependentSet;
pub use maximum_matching::MaximumMatching;
pub use minimum_dominating_set::MinimumDominatingSet;
pub use minimum_vertex_cover::MinimumVertexCover;
pub use spin_glass::SpinGlass;
pub use traveling_salesman::TravelingSalesman;
```

**Step 6: Commit**

```bash
git add src/models/
git commit -m "refactor: update mod.rs files for new category structure"
```

---

### Task 3: Move unit test files to match new source structure

The unit tests mirror the source directory layout under `src/unit_tests/models/`.

**Files:**
- Move: 12 unit test files to new directories
- Create: `src/unit_tests/models/formula/`, `src/unit_tests/models/algebraic/`, `src/unit_tests/models/misc/`

**Step 1: Move test files**

```bash
# formula/ (rename satisfiability/ + add circuit)
git mv src/unit_tests/models/satisfiability src/unit_tests/models/formula
git mv src/unit_tests/models/specialized/circuit.rs src/unit_tests/models/formula/circuit.rs

# algebraic/
mkdir -p src/unit_tests/models/algebraic
git mv src/unit_tests/models/optimization/qubo.rs src/unit_tests/models/algebraic/qubo.rs
git mv src/unit_tests/models/optimization/ilp.rs src/unit_tests/models/algebraic/ilp.rs
git mv src/unit_tests/models/optimization/closest_vector_problem.rs src/unit_tests/models/algebraic/closest_vector_problem.rs
git mv src/unit_tests/models/specialized/bmf.rs src/unit_tests/models/algebraic/bmf.rs

# misc/
mkdir -p src/unit_tests/models/misc
git mv src/unit_tests/models/optimization/bin_packing.rs src/unit_tests/models/misc/bin_packing.rs
git mv src/unit_tests/models/specialized/factoring.rs src/unit_tests/models/misc/factoring.rs
git mv src/unit_tests/models/specialized/paintshop.rs src/unit_tests/models/misc/paintshop.rs

# graph/ (add spin_glass and biclique_cover)
git mv src/unit_tests/models/optimization/spin_glass.rs src/unit_tests/models/graph/spin_glass.rs
git mv src/unit_tests/models/specialized/biclique_cover.rs src/unit_tests/models/graph/biclique_cover.rs

# Remove empty old directories
rmdir src/unit_tests/models/optimization
rmdir src/unit_tests/models/specialized
```

**Step 2: Update `#[path]` attributes in moved source files**

Each model source file has a `#[cfg(test)] #[path = "..."]` attribute pointing to its test file. After the moves, these relative paths are still correct because both source and test moved by the same amount — **except** for files that changed category (e.g., spin_glass moved from `optimization/` to `graph/`).

Update these `#[path]` attributes in the **source** files:

| Source file (new location) | Old `#[path]` | New `#[path]` |
|---|---|---|
| `src/models/formula/sat.rs` | `../../unit_tests/models/satisfiability/sat.rs` | `../../unit_tests/models/formula/sat.rs` |
| `src/models/formula/ksat.rs` | `../../unit_tests/models/satisfiability/ksat.rs` | `../../unit_tests/models/formula/ksat.rs` |
| `src/models/formula/circuit.rs` | `../../unit_tests/models/specialized/circuit.rs` | `../../unit_tests/models/formula/circuit.rs` |
| `src/models/algebraic/qubo.rs` | `../../unit_tests/models/optimization/qubo.rs` | `../../unit_tests/models/algebraic/qubo.rs` |
| `src/models/algebraic/ilp.rs` | `../../unit_tests/models/optimization/ilp.rs` | `../../unit_tests/models/algebraic/ilp.rs` |
| `src/models/algebraic/closest_vector_problem.rs` | `../../unit_tests/models/optimization/closest_vector_problem.rs` | `../../unit_tests/models/algebraic/closest_vector_problem.rs` |
| `src/models/algebraic/bmf.rs` | `../../unit_tests/models/specialized/bmf.rs` | `../../unit_tests/models/algebraic/bmf.rs` |
| `src/models/misc/bin_packing.rs` | `../../unit_tests/models/optimization/bin_packing.rs` | `../../unit_tests/models/misc/bin_packing.rs` |
| `src/models/misc/factoring.rs` | `../../unit_tests/models/specialized/factoring.rs` | `../../unit_tests/models/misc/factoring.rs` |
| `src/models/misc/paintshop.rs` | `../../unit_tests/models/specialized/paintshop.rs` | `../../unit_tests/models/misc/paintshop.rs` |
| `src/models/graph/spin_glass.rs` | `../../unit_tests/models/optimization/spin_glass.rs` | `../../unit_tests/models/graph/spin_glass.rs` |
| `src/models/graph/biclique_cover.rs` | `../../unit_tests/models/specialized/biclique_cover.rs` | `../../unit_tests/models/graph/biclique_cover.rs` |

Use grep to find the exact `#[path` line in each file and update it.

**Step 3: Commit**

```bash
git add -A src/unit_tests/models/ src/models/
git commit -m "refactor: move unit tests to match new category structure"
```

---

### Task 4: Update all `use` imports in `src/rules/`

Every reduction rule file imports problem types via `crate::models::<old_category>::...`. These must be updated to the new category paths.

**Files:** ~27 files in `src/rules/`

**Replacements (find and replace across all files in `src/rules/`):**

| Old import path | New import path |
|---|---|
| `crate::models::optimization::SpinGlass` | `crate::models::graph::SpinGlass` |
| `crate::models::optimization::QUBO` | `crate::models::algebraic::QUBO` |
| `crate::models::optimization::{...ILP...}` | `crate::models::algebraic::{...ILP...}` |
| `crate::models::optimization::BinPacking` | `crate::models::misc::BinPacking` |
| `crate::models::optimization::ClosestVectorProblem` | `crate::models::algebraic::ClosestVectorProblem` |
| `crate::models::satisfiability::Satisfiability` | `crate::models::formula::Satisfiability` |
| `crate::models::satisfiability::KSatisfiability` | `crate::models::formula::KSatisfiability` |
| `crate::models::satisfiability::{CNFClause, ...}` | `crate::models::formula::{CNFClause, ...}` |
| `crate::models::specialized::CircuitSAT` | `crate::models::formula::CircuitSAT` |
| `crate::models::specialized::{Assignment, BooleanExpr, ...Circuit...}` | `crate::models::formula::{Assignment, BooleanExpr, ...Circuit...}` |
| `crate::models::specialized::Factoring` | `crate::models::misc::Factoring` |
| `crate::models::specialized::PaintShop` | `crate::models::misc::PaintShop` |
| `crate::models::specialized::BicliqueCover` | `crate::models::graph::BicliqueCover` |
| `crate::models::specialized::BMF` | `crate::models::algebraic::BMF` |

**Approach:** For each file in `src/rules/`, read it, identify any `use crate::models::{optimization,satisfiability,specialized}` lines, and update them. Many files import from multiple old categories, so some may need imports split into multiple `use` statements.

Specific files to update (grouped by what they import):

**ILP imports (optimization → algebraic):** `ilp_qubo.rs`, `qubo_ilp.rs`, `minimumsetcovering_ilp.rs`, `minimumvertexcover_ilp.rs`, `maximummatching_ilp.rs`, `circuit_ilp.rs`, `coloring_ilp.rs`, `maximumclique_ilp.rs`, `maximumindependentset_ilp.rs`, `maximumsetpacking_ilp.rs`, `travelingsalesman_ilp.rs`, `minimumdominatingset_ilp.rs`, `factoring_ilp.rs`

**QUBO imports (optimization → algebraic):** `spinglass_qubo.rs`, `maximumsetpacking_qubo.rs`, `ksatisfiability_qubo.rs`, `minimumvertexcover_qubo.rs`, `maximumindependentset_qubo.rs`, `coloring_qubo.rs`, `ilp_qubo.rs`

**SpinGlass imports (optimization → graph):** `spinglass_qubo.rs`, `spinglass_maxcut.rs`, `spinglass_casts.rs`, `circuit_spinglass.rs`

**Satisfiability imports (satisfiability → formula):** `sat_coloring.rs`, `sat_ksat.rs`, `sat_maximumindependentset.rs`, `sat_minimumdominatingset.rs`, `sat_circuitsat.rs`, `ksatisfiability_qubo.rs`, `ksatisfiability_casts.rs`

**CircuitSAT/Circuit imports (specialized → formula):** `circuit_spinglass.rs`, `circuit_ilp.rs`, `factoring_circuit.rs`, `sat_circuitsat.rs`

**Factoring imports (specialized → misc):** `factoring_circuit.rs`, `factoring_ilp.rs`

**Step: Commit**

```bash
git add src/rules/
git commit -m "refactor: update rule imports for new model categories"
```

---

### Task 5: Update `src/lib.rs` prelude and doc comments

**Files:**
- Modify: `src/lib.rs`

**Step 1: Update the API overview doc comment (line 10)**

Change:
```rust
//! | [`models`] | Problem types — [`graph`](models::graph), [`satisfiability`](models::satisfiability), [`set`](models::set), [`optimization`](models::optimization), [`specialized`](models::specialized) |
```
To:
```rust
//! | [`models`] | Problem types — [`graph`](models::graph), [`formula`](models::formula), [`set`](models::set), [`algebraic`](models::algebraic), [`misc`](models::misc) |
```

**Step 2: Update the prelude module (lines 43-46)**

Change:
```rust
    pub use crate::models::optimization::{SpinGlass, QUBO};
    pub use crate::models::satisfiability::{CNFClause, KSatisfiability, Satisfiability};
    pub use crate::models::set::{MaximumSetPacking, MinimumSetCovering};
    pub use crate::models::specialized::{BicliqueCover, CircuitSAT, Factoring, PaintShop, BMF};
```
To:
```rust
    pub use crate::models::algebraic::{QUBO, BMF};
    pub use crate::models::formula::{CNFClause, CircuitSAT, KSatisfiability, Satisfiability};
    pub use crate::models::graph::{BicliqueCover, SpinGlass};
    pub use crate::models::misc::{BinPacking, Factoring, PaintShop};
    pub use crate::models::set::{MaximumSetPacking, MinimumSetCovering};
```

**Step 3: Commit**

```bash
git add src/lib.rs
git commit -m "refactor: update lib.rs prelude and docs for new categories"
```

---

### Task 6: Update test imports and category assertions

**Files:**
- Modify: `tests/suites/integration.rs`
- Modify: `tests/suites/reductions.rs`
- Modify: `src/unit_tests/rules/graph.rs`

**Step 1: Update `tests/suites/integration.rs` (lines 7-10)**

Change:
```rust
use problemreductions::models::optimization::*;
use problemreductions::models::satisfiability::*;
use problemreductions::models::set::*;
use problemreductions::models::specialized::*;
```
To:
```rust
use problemreductions::models::algebraic::*;
use problemreductions::models::formula::*;
use problemreductions::models::misc::*;
use problemreductions::models::set::*;
```

**Step 2: Update `tests/suites/reductions.rs` (line 6)**

Change:
```rust
use problemreductions::models::optimization::{LinearConstraint, ObjectiveSense, ILP};
```
To:
```rust
use problemreductions::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
```

**Step 3: Update category assertions in `src/unit_tests/rules/graph.rs`**

| Line | Old assertion | New assertion |
|------|---|---|
| 219 | `n.category == "optimization"` | `n.category == "algebraic"` |
| 271-272 | `"optimization"` | `"algebraic"` |
| 275-276 | `"satisfiability"` | `"formula"` |
| 281-282 | `"specialized"` | `"misc"` |
| 397 | `categories.contains("optimization")` | `categories.contains("algebraic")` |
| 398 | `categories.contains("satisfiability")` | `categories.contains("formula")` |
| 399 | `categories.contains("specialized")` | `categories.contains("misc")` |
| 502 | `circuit.category, "specialized"` | `circuit.category, "formula"` |
| 782-783 | `"satisfiability"` | `"formula"` |
| 790-791 | `"optimization"` | `"algebraic"` |

Also update the module path strings in the test assertions (lines 270-282):
- `"problemreductions::models::optimization::qubo"` → `"problemreductions::models::algebraic::qubo"`
- `"problemreductions::models::satisfiability::sat"` → `"problemreductions::models::formula::sat"`
- `"problemreductions::models::specialized::factoring"` → `"problemreductions::models::misc::factoring"`

And in `test_classify_problem_category` (lines 781-791):
- `"problemreductions::models::satisfiability::satisfiability"` → `"problemreductions::models::formula::satisfiability"`
- `"problemreductions::models::optimization::qubo"` → `"problemreductions::models::algebraic::qubo"`

**Step 4: Commit**

```bash
git add tests/ src/unit_tests/
git commit -m "refactor: update test imports and category assertions"
```

---

### Task 7: Update example imports

**Files:** ~15 files in `examples/`

**Replacements:**

| Old import | New import |
|---|---|
| `problemreductions::models::optimization::ILP` | `problemreductions::models::algebraic::ILP` |
| `problemreductions::models::optimization::{LinearConstraint, ObjectiveSense, ILP}` | `problemreductions::models::algebraic::{LinearConstraint, ObjectiveSense, ILP}` |
| `problemreductions::models::specialized::{Assignment, BooleanExpr, Circuit}` | `problemreductions::models::formula::{Assignment, BooleanExpr, Circuit}` |
| `problemreductions::models::specialized::{Assignment, BooleanExpr, Circuit, CircuitSAT}` | `problemreductions::models::formula::{Assignment, BooleanExpr, Circuit, CircuitSAT}` |

Affected example files:
- All `reduction_*_to_ilp.rs` files (ILP import)
- `reduction_ilp_to_qubo.rs` (ILP import)
- `reduction_circuitsat_to_ilp.rs` (ILP + Circuit imports)
- `reduction_factoring_to_circuitsat.rs` (Circuit import)
- `reduction_circuitsat_to_spinglass.rs` (Circuit import)

**Step: Commit**

```bash
git add examples/
git commit -m "refactor: update example imports for new categories"
```

---

### Task 8: Update documentation references

**Files:**
- Modify: `docs/src/getting-started.md` (line 42)
- Modify: `docs/src/design.md` (module overview table)

**Step 1: Update `docs/src/getting-started.md`**

Change:
```rust
use problemreductions::models::optimization::ILP;
```
To:
```rust
use problemreductions::models::algebraic::ILP;
```

**Step 2: Update `docs/src/design.md` module table** (already done partially in earlier commit — verify the line is correct)

**Step 3: Commit**

```bash
git add docs/
git commit -m "docs: update import paths in getting-started guide"
```

---

### Task 9: Update reduction graph visualization colors

**Files:**
- Modify: `docs/src/static/reduction-graph.js` (lines 16-23)

**Step 1: Update `categoryColors` and `categoryBorders`**

Change:
```javascript
var categoryColors = {
  graph: '#c8f0c8', set: '#f0c8c8', optimization: '#f0f0a0',
  satisfiability: '#c8c8f0', specialized: '#f0c8e0'
};
var categoryBorders = {
  graph: '#4a8c4a', set: '#8c4a4a', optimization: '#8c8c4a',
  satisfiability: '#4a4a8c', specialized: '#8c4a6a'
};
```
To:
```javascript
var categoryColors = {
  graph: '#c8f0c8', set: '#f0c8c8', algebraic: '#f0f0a0',
  formula: '#c8c8f0', misc: '#f0c8e0'
};
var categoryBorders = {
  graph: '#4a8c4a', set: '#8c4a4a', algebraic: '#8c8c4a',
  formula: '#4a4a8c', misc: '#8c4a6a'
};
```

**Step 2: Commit**

```bash
git add docs/src/static/reduction-graph.js
git commit -m "docs: update reduction graph colors for new categories"
```

---

### Task 10: Build, test, and regenerate artifacts

**Step 1: Run `make check` to verify everything compiles and passes**

```bash
make check
```

Expected: All fmt, clippy, and test checks pass. If there are compilation errors, they will point to remaining old import paths that were missed — fix them.

**Step 2: Regenerate the reduction graph JSON (categories are derived from module paths)**

```bash
cargo run --example export_graph
cargo run --example export_schemas
```

This regenerates `docs/src/reductions/reduction_graph.json` and `docs/src/reductions/problem_schemas.json` with the new category values.

**Step 3: Build mdbook to verify visualization**

```bash
make doc
```

**Step 4: Commit regenerated artifacts**

```bash
git add docs/src/reductions/ docs/book/
git commit -m "chore: regenerate reduction graph with new categories"
```

**Step 5: Run full test suite one final time**

```bash
make check
```

Expected: All green.
