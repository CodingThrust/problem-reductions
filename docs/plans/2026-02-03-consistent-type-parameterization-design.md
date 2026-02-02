# Consistent Type Parameterization Design

## Summary

Refactor all problem types to use consistent parameterization patterns:
- Graph problems: `Problem<G, W>` with explicit graph type and weight type
- Const generic K where meaningful: `KColoring<K, G, W>`, `KSatisfiability<K, W>`
- No default type parameters - always explicit

## Motivation

Current state is inconsistent:

| Problem | Current | Issue |
|---------|---------|-------|
| `IndependentSet<W>` | Weight only | No graph type parameter |
| `KSatisfiability<K, W>` | Const K + Weight | Good pattern |
| `Coloring` | No parameters | K is runtime field, should be const generic |
| `SpinGlass<W>` | Weight only | No graph type parameter |

## Design

### Type Signatures

```rust
// Graph problems - vertex selection
pub struct IndependentSet<G, W> { graph: G, weights: Vec<W> }
pub struct VertexCovering<G, W> { graph: G, weights: Vec<W> }
pub struct DominatingSet<G, W>  { graph: G, weights: Vec<W> }
pub struct MaximalIS<G, W>      { graph: G, weights: Vec<W> }

// Graph problems - edge weights
pub struct MaxCut<G, W>   { graph: G, edge_weights: Vec<W> }
pub struct Matching<G, W> { graph: G, edge_weights: Vec<W> }

// Graph problems - const K
pub struct KColoring<const K: usize, G, W> { graph: G, weights: Vec<W> }

// Sparse coupling topology
pub struct SpinGlass<G, W> { graph: G, fields: Vec<W>, couplings: Vec<W> }

// Dense/matrix-based (no graph parameter)
pub struct QUBO<W> { matrix: Vec<Vec<W>> }

// No graph topology
pub struct Satisfiability<W>                  { clauses: Vec<CNFClause>, weights: Vec<W> }
pub struct KSatisfiability<const K: usize, W> { clauses: Vec<CNFClause>, weights: Vec<W> }
pub struct CircuitSAT<W>                      { circuit: Circuit, weights: Vec<W> }
pub struct SetPacking<W>                      { sets: Vec<Vec<usize>>, weights: Vec<W> }
pub struct SetCovering<W>                     { sets: Vec<Vec<usize>>, weights: Vec<W> }
pub struct ILP                                { constraints: Vec<LinearConstraint>, ... }
pub struct Factoring                          { m: usize, n: usize, target: u64 }
```

### Usage Examples

```rust
// Explicit type parameters - no defaults
let is = IndependentSet::<SimpleGraph, i32>::new(4, vec![(0,1), (1,2)]);
let col = KColoring::<3, SimpleGraph, i32>::new(4, vec![(0,1), (1,2)]);
let udg_is = IndependentSet::<UnitDiskGraph, f64>::from_graph(udg, weights);
let sg = SpinGlass::<GridGraph, f64>::new(grid, fields, couplings);
```

### Variant Method

```rust
impl<G: Graph, W: 'static> Problem for IndependentSet<G, W> {
    const NAME: &'static str = "IndependentSet";

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", G::NAME),
            ("weight", short_type_name::<W>()),
        ]
    }
}

impl<const K: usize, G: Graph, W: 'static> Problem for KColoring<K, G, W> {
    const NAME: &'static str = "KColoring";

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("k", const_str::<K>()),
            ("graph", G::NAME),
            ("weight", short_type_name::<W>()),
        ]
    }
}
```

Note: Need `const_str::<K>()` helper to convert const generic to `&'static str`.

### Variant ID Format (Typst only)

Format: `{ProblemName}[/{K}][/{GraphType}][/{WeightType}]`

| Type | Variant ID |
|------|------------|
| `IndependentSet<SimpleGraph, i32>` | `IndependentSet` |
| `IndependentSet<GridGraph, i32>` | `IndependentSet/GridGraph` |
| `IndependentSet<SimpleGraph, f64>` | `IndependentSet/Weighted` |
| `KColoring<3, SimpleGraph, i32>` | `KColoring/3` |
| `KColoring<3, GridGraph, i32>` | `KColoring/3/GridGraph` |
| `SpinGlass<GridGraph, f64>` | `SpinGlass/GridGraph` |

Rules:
- Omit if default: `SimpleGraph`, `i32` (or `f64` for SpinGlass)
- Order: K -> Graph -> Weight
- `Weighted` marker for non-default weight type

## Migration Strategy

Breaking change - no backwards compatibility.

### Step 1: Add G parameter to existing types

Files to modify:
- `src/models/graph/independent_set.rs`
- `src/models/graph/vertex_covering.rs`
- `src/models/graph/dominating_set.rs`
- `src/models/graph/maximal_is.rs`
- `src/models/graph/max_cut.rs`
- `src/models/graph/matching.rs`
- `src/models/optimization/spin_glass.rs`

### Step 2: Rename Coloring -> KColoring with const K

- Delete `src/models/graph/coloring.rs`
- Create `src/models/graph/kcoloring.rs`
- Add const generic K, remove `num_colors` field

### Step 3: Update all reduction implementations

Most reductions are `SimpleGraph` -> `SimpleGraph`, so changes are mechanical:
- `src/rules/*.rs` - update type signatures

### Step 4: Remove template-based types

Delete:
- `GraphProblem<C, G, W>` struct
- `GraphConstraint` trait
- `IndependentSetT`, `VertexCoverT`, `CliqueT` type aliases

File: `src/models/graph/template.rs` - delete or gut

### Step 5: Update documentation

- `docs/paper/reductions.typ` - update type definitions
- `make export-graph` - regenerate reduction graph
- Update examples in doc comments

## Estimated Scope

- ~15 source files
- ~50 test files (update type annotations)
- Mostly mechanical changes
- Breaking API change (major version bump)

## Alternatives Considered

1. **Template-based types only** - Rejected: verbose API, complex error messages
2. **Default type parameters** - Rejected: prefer explicit over implicit
3. **Runtime K for Coloring** - Rejected: inconsistent with KSatisfiability pattern
