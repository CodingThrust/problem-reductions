# Feature Parity Design: ProblemReductions.jl → Rust

## Overview

Port all remaining features from the Julia `ProblemReductions.jl` package to complete feature parity with the Rust implementation.

## Current State

| Category | Status |
|----------|--------|
| Problem Models (18) | Complete |
| BruteForce Solver | Complete |
| Test Coverage | 97.53% (above 95% target) |

## Features to Implement

### 1. Reduction Rules Framework

**Design: Hybrid approach (compile-time traits + runtime registry)**

#### Core Traits

```rust
// src/rules/traits.rs

/// Result of a reduction from Source to Target problem
pub trait ReductionResult: Clone {
    type Source: Problem;
    type Target: Problem;

    /// Get the target problem
    fn target_problem(&self) -> &Self::Target;

    /// Extract solution from target back to source
    fn extract_solution(&self, target_sol: &[usize]) -> Vec<usize>;

    /// Problem sizes for complexity analysis
    fn source_size(&self) -> ProblemSize;
    fn target_size(&self) -> ProblemSize;
}

/// Trait for problems that can be reduced to type T
pub trait ReduceTo<T: Problem>: Problem {
    type Result: ReductionResult<Source = Self, Target = T>;
    fn reduce_to(&self) -> Self::Result;
}
```

#### Runtime Registry

```rust
// src/rules/graph.rs

use petgraph::graph::DiGraph;

pub struct ReductionGraph {
    graph: DiGraph<TypeId, ()>,
    type_names: HashMap<TypeId, &'static str>,
    reducers: HashMap<(TypeId, TypeId), Box<dyn DynReducer>>,
}

impl ReductionGraph {
    /// Build graph from all registered reductions
    pub fn new() -> Self;

    /// Find all paths from source to target problem type
    pub fn find_paths<S: Problem, T: Problem>(&self) -> Vec<ReductionPath>;

    /// Execute a reduction path
    pub fn reduce_along_path<S: Problem>(
        &self,
        problem: &S,
        path: &ReductionPath
    ) -> ConcatenatedReduction;
}

/// A path through the reduction graph
pub struct ReductionPath {
    pub type_names: Vec<&'static str>,
    pub type_ids: Vec<TypeId>,
}

/// Chain of reductions with solution extraction
pub struct ConcatenatedReduction {
    steps: Vec<Box<dyn AnyReductionResult>>,
}

impl ConcatenatedReduction {
    pub fn target_problem(&self) -> &dyn Problem;
    pub fn extract_solution(&self, final_sol: &[usize]) -> Vec<usize>;
}
```

#### 14 Reduction Rules to Implement

| Rule | Direction | File |
|------|-----------|------|
| SpinGlass ↔ SAT | Bidirectional | `spinglass_sat.rs` |
| SpinGlass ↔ MaxCut | Bidirectional | `spinglass_maxcut.rs` |
| SpinGlass ↔ QUBO | Bidirectional | `spinglass_qubo.rs` |
| SAT → 3SAT | One-way | `sat_3sat.rs` |
| SAT → Coloring | One-way | `sat_coloring.rs` |
| SAT → IndependentSet | One-way | `sat_independentset.rs` |
| SAT → DominatingSet | One-way | `sat_dominatingset.rs` |
| VertexCovering → SetCovering | One-way | `vertexcovering_setcovering.rs` |
| VertexCovering ↔ IndependentSet | Bidirectional | `vertexcovering_independentset.rs` |
| IndependentSet ↔ SetPacking | Bidirectional | `independentset_setpacking.rs` |
| Matching → SetPacking | One-way | `matching_setpacking.rs` |
| Circuit → SAT | One-way | `circuit_sat.rs` |
| Factoring → SAT | One-way | `factoring_sat.rs` |

### 2. Topology Types

**Design: Custom types (petgraph doesn't support hyperedges or implicit geometry)**

```rust
// src/topology/hypergraph.rs

/// Hypergraph where edges can connect any number of vertices
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HyperGraph {
    num_vertices: usize,
    edges: Vec<Vec<usize>>,
}

impl HyperGraph {
    pub fn new(num_vertices: usize, edges: Vec<Vec<usize>>) -> Self;
    pub fn num_vertices(&self) -> usize;
    pub fn num_edges(&self) -> usize;
    pub fn edges(&self) -> &[Vec<usize>];
    pub fn has_edge(&self, edge: &[usize]) -> bool;
    pub fn neighbors(&self, v: usize) -> Vec<usize>;
}
```

```rust
// src/topology/unit_disk_graph.rs

/// Graph defined by vertex locations and distance threshold
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnitDiskGraph<const D: usize = 2> {
    locations: Vec<[f64; D]>,
    radius: f64,
}

impl<const D: usize> UnitDiskGraph<D> {
    pub fn new(locations: Vec<[f64; D]>, radius: f64) -> Self;
    pub fn num_vertices(&self) -> usize;
    pub fn has_edge(&self, i: usize, j: usize) -> bool;
    pub fn neighbors(&self, i: usize) -> Vec<usize>;
    pub fn edges(&self) -> Vec<(usize, usize)>;
    pub fn to_petgraph(&self) -> UnGraph<(), ()>;
}

/// UnitDiskGraph with integer coordinates
pub type GridGraph<const D: usize = 2> = UnitDiskGraph<D>;
```

### 3. TruthTable

```rust
// src/truth_table.rs

/// Truth table for logic functions
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TruthTable<T: Clone = String> {
    inputs: Vec<T>,
    outputs: Vec<T>,
    values: Vec<u64>,  // Bit-packed output values per input combination
}

impl<T: Clone> TruthTable<T> {
    pub fn new(inputs: Vec<T>, outputs: Vec<T>, values: Vec<u64>) -> Self;
    pub fn evaluate(&self, input_bits: u64) -> u64;
    pub fn num_inputs(&self) -> usize;
    pub fn num_outputs(&self) -> usize;
}

impl<T: Clone + Display> Display for TruthTable<T> {
    // Pretty-print as table
}
```

### 4. File I/O

```rust
// src/io.rs

use serde::{Serialize, de::DeserializeOwned};
use std::path::Path;

/// Save any serializable problem to JSON
pub fn save_json<P: Serialize>(problem: &P, path: impl AsRef<Path>) -> Result<()>;

/// Load problem from JSON
pub fn load_json<P: DeserializeOwned>(path: impl AsRef<Path>) -> Result<P>;
```

## File Structure

```
src/
├── lib.rs
├── traits.rs
├── types.rs
├── config.rs
├── error.rs
├── io.rs                     # NEW
├── truth_table.rs            # NEW
├── models/                   # Existing (complete)
├── solvers/                  # Existing (complete)
├── topology/                 # NEW
│   ├── mod.rs
│   ├── hypergraph.rs
│   └── unit_disk_graph.rs
└── rules/                    # NEW
    ├── mod.rs
    ├── traits.rs
    ├── graph.rs
    ├── concatenated.rs
    ├── spinglass_sat.rs
    ├── spinglass_maxcut.rs
    ├── spinglass_qubo.rs
    ├── sat_3sat.rs
    ├── sat_coloring.rs
    ├── sat_independentset.rs
    ├── sat_dominatingset.rs
    ├── vertexcovering_setcovering.rs
    ├── vertexcovering_independentset.rs
    ├── independentset_setpacking.rs
    ├── matching_setpacking.rs
    ├── circuit_sat.rs
    └── factoring_sat.rs
```

## Implementation Phases

### Phase 1: Core Infrastructure
- [ ] Create `src/rules/mod.rs` with core traits
- [ ] Create `src/rules/traits.rs` with `ReductionResult` and `ReduceTo`
- [ ] Create `src/rules/graph.rs` with `ReductionGraph`
- [ ] Create `src/rules/concatenated.rs` with `ConcatenatedReduction`

### Phase 2: Reduction Rules (ordered by dependency)
- [ ] `vertexcovering_independentset.rs` (simplest, good template)
- [ ] `independentset_setpacking.rs`
- [ ] `matching_setpacking.rs`
- [ ] `vertexcovering_setcovering.rs`
- [ ] `spinglass_qubo.rs`
- [ ] `spinglass_maxcut.rs`
- [ ] `sat_3sat.rs`
- [ ] `sat_independentset.rs`
- [ ] `sat_coloring.rs`
- [ ] `sat_dominatingset.rs`
- [ ] `spinglass_sat.rs`
- [ ] `circuit_sat.rs`
- [ ] `factoring_sat.rs`

### Phase 3: Topology
- [ ] Create `src/topology/mod.rs`
- [ ] Implement `HyperGraph`
- [ ] Implement `UnitDiskGraph` / `GridGraph`

### Phase 4: Utilities
- [ ] Implement `TruthTable`
- [ ] Implement File I/O (`save_json`, `load_json`)

### Phase 5: Testing
- [ ] Unit tests for each reduction rule
- [ ] Integration tests for reduction chains
- [ ] Tests for topology types
- [ ] Tests for TruthTable and I/O
- [ ] Maintain >95% coverage

## Dependencies

No new dependencies required. Current deps are sufficient:
- `petgraph` — for `ReductionGraph` path finding
- `serde` — for File I/O
- `bitvec` — for bit operations
- `num-traits` — for numeric generics

## Success Criteria

- [ ] All 14 reduction rules implemented with tests
- [ ] `ReductionGraph` can discover paths between any connected problem types
- [ ] `ConcatenatedReduction` correctly extracts solutions through chains
- [ ] Topology types (HyperGraph, UnitDiskGraph) working with serde
- [ ] TruthTable utility complete
- [ ] File I/O for all problem types
- [ ] Test coverage remains >95%
