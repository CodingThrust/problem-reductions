# Problem Categorization Redesign

## Problem

The current categorization mixes two axes:
- **Input structure** (graph, set system, formula)
- **Problem type** (optimization vs satisfaction)

This creates ambiguity: MIS is both a graph problem and an optimization problem. QUBO is on a matrix but lives in `optimization/`. CircuitSAT is a satisfiability problem but lives in `specialized/`. The `specialized/` folder is a catch-all with no unifying principle.

## Design

**Single axis: primary input structure** — "what data type does the problem operate on?"

The optimization/satisfaction distinction is already captured by the trait hierarchy (`OptimizationProblem` vs `SatisfactionProblem`), so folders should not duplicate it.

### New folder structure

```
src/models/
├── graph/          # Input: a graph (optionally weighted)
│   ├── maximum_independent_set.rs
│   ├── maximum_clique.rs
│   ├── max_cut.rs
│   ├── maximum_matching.rs
│   ├── minimum_vertex_cover.rs
│   ├── minimum_dominating_set.rs
│   ├── maximal_is.rs
│   ├── kcoloring.rs
│   ├── traveling_salesman.rs
│   ├── spin_glass.rs          ← from optimization/
│   └── biclique_cover.rs      ← from specialized/
│
├── formula/        # Input: a logical formula or circuit
│   ├── sat.rs
│   ├── ksat.rs
│   └── circuit.rs             ← from specialized/
│
├── set/            # Input: universe + collection of subsets
│   ├── minimum_set_covering.rs
│   └── maximum_set_packing.rs
│
├── algebraic/      # Input: matrix, linear system, or lattice
│   ├── qubo.rs                ← from optimization/
│   ├── ilp.rs                 ← from optimization/
│   ├── closest_vector_problem.rs  ← from optimization/
│   └── bmf.rs                 ← from specialized/
│
└── misc/           # Problems with unique input structures
    ├── bin_packing.rs         ← from optimization/
    ├── paintshop.rs           ← from specialized/
    └── factoring.rs           ← from specialized/
```

### Decision rule for new problems

> "What is the primary data structure in the struct definition?"
> - Graph → `graph/`
> - Boolean formula or circuit → `formula/`
> - Universe + subsets → `set/`
> - Matrix, linear system, or lattice → `algebraic/`
> - None of the above → `misc/`

### What moves

| Problem | From | To | Reason |
|---|---|---|---|
| SpinGlass | optimization/ | graph/ | Parameterized by G, operates on graph edges |
| BicliqueCover | specialized/ | graph/ | Input is a BipartiteGraph |
| CircuitSAT | specialized/ | formula/ | Input is a boolean circuit |
| QUBO | optimization/ | algebraic/ | Input is a Q matrix (no graph param) |
| ILP | optimization/ | algebraic/ | Input is constraint matrix + objective |
| CVP | optimization/ | algebraic/ | Input is lattice basis matrix |
| BMF | specialized/ | algebraic/ | Input is a boolean matrix |
| BinPacking | optimization/ | misc/ | Input is items + capacity |
| PaintShop | specialized/ | misc/ | Input is a car sequence |
| Factoring | specialized/ | misc/ | Input is an integer |

### What doesn't change

- Trait hierarchy (`OptimizationProblem`, `SatisfactionProblem`)
- All public API, type names, re-exports
- Only `mod.rs` files, `use` paths, and `#[path]` test references change
- The `optimization/` and `specialized/` folders are eliminated
