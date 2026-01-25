# Graph Problems

Graph problems are optimization problems defined on graph structures. This library provides a **template-based** approach where all graph problems share common infrastructure while having problem-specific constraints.

## Design Overview

All graph problems use the `GraphProblem<C, G, W>` template:

- `C: GraphConstraint` - Defines the problem-specific constraint (e.g., independent set, vertex cover)
- `G: Graph` - The graph topology (defaults to `SimpleGraph`)
- `W` - Weight type (defaults to `i32`)

Type aliases provide convenient access:

```rust
use problemreductions::prelude::*;
use problemreductions::topology::{SimpleGraph, UnitDiskGraph};

// These are equivalent - defaults to SimpleGraph and i32 weights
let problem: IndependentSetT = IndependentSetT::new(4, vec![(0, 1), (1, 2)]);
let problem: IndependentSetT<SimpleGraph, i32> = IndependentSetT::new(4, vec![(0, 1), (1, 2)]);

// With custom weight type
let weighted: IndependentSetT<SimpleGraph, f64> = IndependentSetT::with_weights(
    4,
    vec![(0, 1), (1, 2)],
    vec![1.0, 2.0, 3.0, 4.0]
);

// On different graph topology (e.g., for quantum hardware)
let udg = UnitDiskGraph::new(vec![(0.0, 0.0), (1.0, 0.0), (2.0, 0.0)], 1.5);
let quantum_problem: IndependentSetT<UnitDiskGraph> = IndependentSetT::from_graph(udg);
```

See [Graph Topology](../topology.md) for details on graph types.

## IndependentSetT

Find a maximum weight set of vertices where no two are adjacent.

```rust
use problemreductions::prelude::*;

let problem: IndependentSetT = IndependentSetT::new(4, vec![(0, 1), (1, 2), (2, 3)]);
let solver = BruteForce::new();
let solutions = solver.find_best(&problem);
// Maximum IS on a path of 4 vertices has size 2 (vertices 0,2 or 1,3)
```

**Constraint**: For each edge (u, v), at most one of u or v can be selected.

**Objective**: Maximize sum of weights of selected vertices.

## VertexCoverT

Find a minimum weight set of vertices that covers all edges.

```rust
use problemreductions::prelude::*;

let problem: VertexCoverT = VertexCoverT::new(4, vec![(0, 1), (1, 2), (2, 3)]);
let solver = BruteForce::new();
let solutions = solver.find_best(&problem);
// Minimum vertex cover on a path of 4 vertices has size 2
```

**Constraint**: For each edge (u, v), at least one of u or v must be selected.

**Objective**: Minimize sum of weights of selected vertices.

**Relationship with Independent Set**: IS and VC are complements. For any graph with n vertices:
```
|max IS| + |min VC| = n
```

A set S is an independent set if and only if V \ S is a vertex cover.

## CliqueT

Find a maximum weight complete subgraph.

```rust
use problemreductions::prelude::*;

let problem: CliqueT = CliqueT::new(4, vec![(0, 1), (0, 2), (1, 2), (2, 3)]);
// Vertices 0, 1, 2 form a triangle (clique of size 3)
```

**Constraint**: Every pair of selected vertices must be connected by an edge.

**Objective**: Maximize sum of weights of selected vertices.

**Relationship with Independent Set**: Finding a maximum clique in G is equivalent to finding a maximum independent set in the complement graph G'.

## MaxCut

Partition vertices into two sets to maximize the weight of edges between them.

```rust
use problemreductions::prelude::*;

let problem = MaxCut::new(3, vec![(0, 1, 2), (1, 2, 3), (0, 2, 1)]);
// Edge weights: (0,1)=2, (1,2)=3, (0,2)=1
```

**Note**: MaxCut uses a different structure as it requires edge weights directly.

## Coloring

Assign k colors to vertices minimizing adjacent same-color pairs.

```rust
use problemreductions::prelude::*;

let problem = Coloring::new(4, 3, vec![(0, 1), (1, 2), (2, 3)]);
// 4 vertices, 3 colors available
```

## Available Graph Problems

| Problem | Type Alias | Objective | Constraint |
|---------|------------|-----------|------------|
| Independent Set | `IndependentSetT<G, W>` | Maximize | No adjacent selected |
| Vertex Cover | `VertexCoverT<G, W>` | Minimize | All edges covered |
| Clique | `CliqueT<G, W>` | Maximize | All selected adjacent |
| Max Cut | `MaxCut` | Maximize | Cut weight |
| Coloring | `Coloring` | Minimize | Adjacent same-color |
| Dominating Set | `DominatingSet` | Minimize | All dominated |
| Maximal IS | `MaximalIS` | - | Maximal independent |
| Matching | `Matching` | Maximize | Non-adjacent edges |

## Using with Different Topologies

The template design enables **topology-aware reductions**. For example, reducing SAT to Independent Set can target different graph types:

```rust
use problemreductions::topology::{SimpleGraph, UnitDiskGraph};
use problemreductions::models::graph::IndependentSetT;

// Standard reduction produces arbitrary graph
// impl ReduceTo<IndependentSetT<SimpleGraph>> for Satisfiability { ... }

// Topology-aware reduction produces unit disk graph
// impl ReduceTo<IndependentSetT<UnitDiskGraph>> for Satisfiability { ... }
```

This is particularly useful for quantum computing where neutral atom arrays naturally implement unit disk graph connectivity.

## Common Operations

All graph problems implement the `OptProblem` trait:

```rust
use problemreductions::prelude::*;

let problem: IndependentSetT = IndependentSetT::new(4, vec![(0, 1), (1, 2), (2, 3)]);

// Check solution validity and size
let solution = vec![1, 0, 1, 0];  // Select vertices 0 and 2
let result = problem.solution_size(&solution);
assert!(result.is_valid);
assert_eq!(result.size, 2);

// Get problem parameters
assert_eq!(problem.num_variables(), 4);
assert_eq!(problem.num_flavors(), 2);  // Binary: selected (1) or not (0)

// Check optimization direction
assert!(problem.energy_mode().is_maximization());
```

## CSP Interface

Graph problems also implement the `ConstraintSatisfactionProblem` trait:

```rust
use problemreductions::prelude::*;

let problem: IndependentSetT = IndependentSetT::new(3, vec![(0, 1), (1, 2)]);

// Get constraints (one per edge)
let constraints = problem.constraints();
assert_eq!(constraints.len(), 2);

// Get objectives (one per vertex)
let objectives = problem.objectives();
assert_eq!(objectives.len(), 3);

// Check if solution satisfies all constraints
let solution = vec![1, 0, 1];
assert!(problem.is_satisfied(&solution));
```
