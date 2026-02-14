# Developer API Reference

This document provides comprehensive API documentation for developers extending the problem-reductions library.

## Table of Contents

**Fundamentals**
0. [Conceptual Overview](#0-conceptual-overview)
0a. [Design Philosophy](#0a-design-philosophy)
0b. [How Everything Fits Together](#0b-how-everything-fits-together)

**API Documentation**
1. [Core Traits](#1-core-traits)
2. [Problem Parametrization](#2-problem-parametrization)
3. [Graph Topologies](#3-graph-topologies)
4. [Problem Models](#4-problem-models)
5. [Reductions System](#5-reductions-system)
6. [Solution Representation](#6-solution-representation)
7. [Error Handling](#7-error-handling)

**Developer Guide**
8. [Common Patterns & Workflows](#8-common-patterns--workflows)
9. [Extension Guide](#9-extension-guide)
10. [Internal Modules](#10-internal-modules)
11. [Testing Utilities](#11-testing-utilities)

**Advanced Topics**
12. [Performance Considerations](#12-performance-considerations)
13. [Known Limitations](#13-known-limitations)
14. [FAQ & Troubleshooting](#14-faq--troubleshooting)
15. [Complete End-to-End Example](#15-complete-end-to-end-example)

**CLI Tool**
16. [CLI Tool (`pred`)](#16-cli-tool-pred)

---

## 0. Conceptual Overview

### What is problem-reductions?

This library solves one fundamental problem in computational complexity: **establishing relationships between different NP-hard problems**.

The key insight is that NP-hard problems are equivalent in a specific sense:
- **Polynomial reduction**: If problem A can be reduced to problem B in polynomial time, then solving B gives us a solution to A
- **Hardness**: If any NP-hard problem is solvable in polynomial time, all NP-hard problems are
- **Practical value**: Understanding reductions helps us transfer algorithms and insights between problems

### The Two-Pillar Architecture

The library has two main parts working together:

**1. Problem Definitions** (what to solve)
```
MaximumIndependentSet<SimpleGraph, i32>
    ↓ describes
A problem instance with variables, constraints, objective
    ↓ can be
Solved by BruteForce, or reduced to another problem
```

**2. Reductions** (how to transform)
```
MaximumIndependentSet --reduce--> MinimumVertexCover
    ↓ via
ReduceTo trait
    ↓ provides
Variable mapping: IS_solution --extract--> VC_solution
```

### Mental Model: Three Levels

```
┌─────────────────────────────────────────────────┐
│ ABSTRACTION (What developer thinks about)      │
│ "I have an Independent Set problem"             │
└──────────────┬──────────────────────────────────┘
               │
┌──────────────▼──────────────────────────────────┐
│ TRAIT LEVEL (Rust contract)                    │
│ Problem trait: dims(), evaluate()               │
│ ReduceTo trait: reduce_to(), extract_solution() │
└──────────────┬──────────────────────────────────┘
               │
┌──────────────▼──────────────────────────────────┐
│ CONCRETE (Actual data)                         │
│ MaximumIndependentSet<SimpleGraph, i32>         │
│   - graph: SimpleGraph                         │
│   - weights: Vec<i32>                          │
└─────────────────────────────────────────────────┘
```

### Key Concepts You'll Encounter

| Concept | Meaning | Example |
|---------|---------|---------|
| **Problem** | A decision/optimization task with variables and constraints | "Find the maximum independent set" |
| **Configuration** | An assignment of values to all variables | `[1, 0, 1, 0, 1]` for 5 boolean variables |
| **Metric** | The evaluation result of a configuration | `SolutionSize::Valid(42)` or `bool` |
| **Variant** | Dimensions describing a problem type | `[("graph", "SimpleGraph"), ("weight", "i32")]` |
| **Reduction** | Polynomial-time transformation from problem A to problem B | SAT → MaximumIndependentSet |
| **Overhead** | How problem size grows during reduction | n' = n + 10m (n vars, m clauses) |

---

## 0a. Design Philosophy

### Why Generic Over Graph Topologies?

**Problem**: Many graph problems exist on different graph structures:
- Simple graphs (general case)
- Grid graphs (VLSI, image processing)
- Unit disk graphs (wireless networks)
- Hypergraphs (set systems)

**Solution**: Don't rewrite the same problem logic for each topology. Instead:

```rust
// One problem definition works for ALL topologies
pub struct MaximumIndependentSet<G, W> {
    graph: G,          // Can be SimpleGraph, GridGraph, UnitDiskGraph, ...
    weights: Vec<W>,   // Can be i32, f64, One, ...
}
```

**Benefits**:
- ✅ Code reuse
- ✅ Type safety (compiler checks graph compatibility)
- ✅ Monomorphization (no runtime overhead)

### Why Separate Types for Weight?

**Problem**: Problems exist in both weighted and unweighted forms:
- Unweighted: all vertices have weight 1 (simpler, sometimes restricted variant)
- Weighted: vertices have different weights (general case)

**Solution**: Use the weight type as a parameter:

```rust
MaximumIndependentSet::<SimpleGraph, One>::new(5, edges)   // Unweighted variant
MaximumIndependentSet::<SimpleGraph, i32>::new(5, edges)   // Weighted variant (all weights = 1 initially)
MaximumIndependentSet::<SimpleGraph, i32>::with_weights(5, edges, vec![1,2,3,4,5])  // Custom weights
```

The `One` type is a unit weight marker where `One::to_sum()` always returns `1i32`. The type alias `Unweighted = One` is also available.

**Benefits**:
- ✅ Semantic clarity: type system enforces which variant you're using
- ✅ Metadata: variant() method can report exact variant for reduction graph
- ✅ Specialization: can optimize for `One` if needed

### Why Traits Instead of Inheritance?

Rust doesn't have traditional OOP inheritance, so we use traits to define contracts:

```rust
pub trait Problem: Clone {
    type Metric: Clone;
    fn dims(&self) -> Vec<usize>;
    fn evaluate(&self, config: &[usize]) -> Self::Metric;
    // ...
}

// Now we can write generic code that works with ANY problem type!
fn solve_any_problem<P: Problem>(p: &P) -> Option<Vec<usize>> {
    // ...
}
```

**Why this matters**:
- ✅ Extensible: add new problems without modifying library code
- ✅ Generic: write reduction code once, works for all graph types
- ✅ Type-safe: compiler ensures problems implement required methods

### Variant System Explained

The `variant()` method returns metadata about a problem instance:

```rust
impl Problem for MaximumIndependentSet<SimpleGraph, i32> {
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", "SimpleGraph"),      // What graph topology?
            ("weight", "i32"),             // What weight type?
        ]
    }
}
```

**Why this exists**:
1. **Reduction graph**: The library builds a graph of all possible problem variants and reductions
2. **Runtime metadata**: Know what variant you're working with without exposing Rust types
3. **Documentation**: Automatically discover which problems are available and how they relate

**Think of it like**:
```
Problem name: "MaximumIndependentSet"
├── Variant 1: graph=SimpleGraph, weight=One
├── Variant 2: graph=SimpleGraph, weight=i32
├── Variant 3: graph=GridGraph, weight=i32
└── Variant 4: graph=UnitDiskGraph, weight=f64
```

---

## 0b. How Everything Fits Together

This diagram shows how all the pieces interconnect:

```
                    ┌─────────────────────────────────────┐
                    │   USER CODE                         │
                    │  (Your application)                 │
                    └──────────────┬──────────────────────┘
                                   │
                    ┌──────────────▼──────────────────────┐
                    │   PROBLEM INSTANCES                 │
                    │  MaximumIndependentSet<SG, i32>     │
                    │  MinimumVertexCover<SG, i32>        │
                    │  Satisfiability                     │
                    └──────────────┬──────────────────────┘
                                   │
                ┌──────────────────┼──────────────────────┐
                │                  │                      │
        ┌───────▼────────┐  ┌──────▼──────┐  ┌───────────▼───────┐
        │   SOLVERS      │  │ REDUCTIONS  │  │  INTROSPECTION    │
        │                │  │             │  │                   │
        │ - BruteForce   │  │ ReduceTo    │  │ - variant()       │
        │ - ILPSolver    │  │ <Target>    │  │ - dims()          │
        │ - Custom       │  │             │  │ - NAME constant   │
        └───────┬────────┘  └──────┬──────┘  └───────────┬───────┘
                │                  │                      │
                │      ┌───────────▼──────────────┐       │
                └─────▶│   TRAIT SYSTEM          │◀──────┘
                       │  (Core abstraction)     │
                       │  - Problem              │
                       │  - OptimizationProblem  │
                       │  - SatisfactionProblem  │
                       │  - ReduceTo             │
                       │  - Solver               │
                       └───────────┬──────────────┘
                                   │
                       ┌───────────▼──────────────┐
                       │   REDUCTION GRAPH        │
                       │  (Built at startup)      │
                       │                          │
                       │  Nodes:                  │
                       │   MIS[SimpleGraph, i32]  │
                       │   MVC[SimpleGraph, i32]  │
                       │   SAT                    │
                       │                          │
                       │  Edges:                  │
                       │   MIS ◀─complement─▶ MVC │
                       │   SAT ─reduce─▶ MIS      │
                       │   ...                    │
                       └──────────────────────────┘
```

### Lifecycle of a Reduction Query

When you call `.reduce_to()`, here's what happens internally:

```
User code:
  let reduction = source.reduce_to();
  // type inferred: ReduceTo<TargetProblem>

⬇️  Rust compiler searches for impl ReduceTo<TargetProblem> for SourceProblem

⬇️  If found, the impl is called:
    fn reduce_to(&self) -> Self::Result {
        // Construct target problem from source
        // Set up solution extraction mapping
        // Return result type
    }

⬇️  Result type provides:
    - target_problem(): reference to constructed problem
    - extract_solution(target_sol): map back to source

⬇️  Your code can now:
    // Solve target
    let target_solution = solver.find_best(target_problem);

    // Extract source solution
    let source_sol = reduction.extract_solution(&target_solution);
    // source_sol is now valid for original problem!
```

### Where Variant Information Flows

```
Problem Type Definition:
  impl Problem for MaximumIndependentSet<SimpleGraph, i32> {
      fn variant() -> Vec<(&str, &str)> {
          vec![("graph", "SimpleGraph"), ("weight", "i32")]
      }
  }

⬇️  Inventory System (at compile time):
  Collects all Problem implementations
  Extracts their variant() info
  Builds reduction metadata

⬇️  Reduction Graph (at runtime):
  Nodes represent [Problem + Variant] pairs:
    - MIS[SimpleGraph, i32]
    - MIS[SimpleGraph, f64]
    - MIS[GridGraph, i32]
    - MVC[SimpleGraph, i32]
    - ...

  Edges represent available reductions:
    - MIS[SimpleGraph, i32] --complement--> MVC[SimpleGraph, i32]
    - MIS[SimpleGraph, i32] --to-set-packing--> SetPacking[...]
    - ...

⬇️  Your Application:
  Can query: "What problems can I reduce this to?"
  Answer: "MVC[SimpleGraph, i32], SetPacking[...], ..."

  Can query: "What's the overhead of reducing A to B?"
  Answer: "num_vars: 1n + 0m, num_constraints: 5m"
```

---

## 1. Core Traits

This section explains the fundamental traits that define the contract for problem types.

### Understanding Trait Hierarchy

```
┌─────────────────────────────────────┐
│      Problem (base trait)           │
│  - const NAME                       │
│  - type Metric                      │
│  - dims(), evaluate()               │
│  - num_variables() (derived)        │
│  - variant()                        │
└──────────────┬──────────────────────┘
               │
    ┌──────────┴──────────────────────────────────┐
    │                                             │
┌───▼───────────────────────────┐  ┌──────────────▼───────────────┐
│ OptimizationProblem           │  │ SatisfactionProblem          │
│ (Metric = SolutionSize<V>)    │  │ (Metric = bool)              │
│  + type Value                 │  │ (marker trait, no methods)    │
│  + direction()                │  │                              │
└───────────────────────────────┘  └──────────────────────────────┘

Solver
  - find_best() for OptimizationProblem
  - find_satisfying() for SatisfactionProblem

ReduceTo<T>
  - reduce_to() → ReductionResult
```

**Key relationship**:
- All problems implement `Problem`
- Optimization problems additionally implement `OptimizationProblem`
- Decision problems additionally implement `SatisfactionProblem`
- Solvers dispatch on the problem type
- Reductions connect problem types

### 1.1 Problem Trait

**Location**: `src/traits.rs`

The foundational trait that all problems must implement.

```rust
pub trait Problem: Clone {
    /// Base name of this problem type (e.g., "MaximumIndependentSet").
    const NAME: &'static str;

    /// The evaluation metric type.
    type Metric: Clone;

    /// Configuration space dimensions. Each entry is the cardinality of that variable.
    fn dims(&self) -> Vec<usize>;

    /// Evaluate the problem on a configuration.
    fn evaluate(&self, config: &[usize]) -> Self::Metric;

    /// Number of variables (derived from dims).
    fn num_variables(&self) -> usize {
        self.dims().len()
    }

    /// Returns variant attributes derived from type parameters.
    fn variant() -> Vec<(&'static str, &'static str)>;
}
```

#### Associated Types

| Type | Bounds | Purpose |
|------|--------|---------|
| `Metric` | `Clone` | Evaluation result type — `SolutionSize<W::Sum>` for optimization, `bool` for satisfaction |

#### Required Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `dims()` | `Vec<usize>` | Configuration space dimensions (e.g., `[2, 2, 2]` for 3 binary vars) |
| `evaluate(config)` | `Self::Metric` | Evaluate a configuration |
| `variant()` | `Vec<(&str, &str)>` | Describe problem variant (graph type, weight type) |

#### Provided Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `num_variables()` | `usize` | Number of decision variables (default: `dims().len()`) |

#### Contract

- Configuration length must equal `num_variables()` (i.e., `dims().len()`)
- Each configuration value at index `i` must be in `0..dims()[i]`
- `evaluate()` must never panic on valid configs
- Invalid configs: behavior is implementation-defined (may return `SolutionSize::Invalid`, `false`, etc.)

#### Variant Method Deep Dive

The `variant()` method is **static** (uses `Self::` not `&self`) and returns metadata about problem variants:

```rust
impl Problem for MaximumIndependentSet<SimpleGraph, i32> {
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i32")]
    }
}
```

**Why static?** Because we need variant info *before* creating a problem instance. This lets us:
1. Build a reduction graph at startup (knowing all available problems)
2. Match reductions without instantiating problems
3. Provide introspection without overhead

**Common variant keys**:
- `"graph"` - Graph topology type (e.g., "SimpleGraph", "GridGraph", "UnitDiskGraph")
- `"weight"` - Weight type (e.g., "i32", "f64", "One")

**How variants are used internally**:
```rust
// The library builds this mapping:
"MaximumIndependentSet" has variants:
  - [("graph", "SimpleGraph"), ("weight", "One")]
  - [("graph", "SimpleGraph"), ("weight", "i32")]
  - [("graph", "SimpleGraph"), ("weight", "f64")]
  - [("graph", "GridGraph"), ("weight", "i32")]
  - ...

Then it finds which reductions are available:
  - MIS[SimpleGraph, i32] --complement--> MVC[SimpleGraph, i32]
  - MIS[SimpleGraph, i32] --subset--> SetPacking[...]
  - ...
```

#### Creating Configurations

Understanding how configurations are created and what they represent:

```rust
// Binary problem: dims() = [2, 2, 2, 2, 2]
// Variables represent yes/no decisions
let config = vec![1, 0, 1, 0, 1];  // Five binary decisions
// config[0] = 1: select vertex 0
// config[1] = 0: don't select vertex 1
// config[2] = 1: select vertex 2
// etc.

// Multi-flavor problem: dims() = [3, 3, 3, 3, 3] (e.g., k-coloring with k=3)
// Variables represent k-way choices
let config = vec![0, 1, 2, 1, 0];  // Five vertices, up to 3 colors
// config[0] = 0: color vertex 0 with color 0
// config[1] = 1: color vertex 1 with color 1
// etc.
```

### 1.2 OptimizationProblem Trait

**Location**: `src/traits.rs`

Extension for problems with a numeric objective to optimize.

```rust
pub trait OptimizationProblem: Problem<Metric = SolutionSize<Self::Value>> {
    /// The inner objective value type (e.g., `i32`, `f64`).
    type Value: PartialOrd + Clone;

    /// Whether to maximize or minimize the metric.
    fn direction(&self) -> Direction;
}
```

The supertrait bound guarantees `Metric = SolutionSize<Self::Value>`, so the solver can call `metric.is_valid()` and `metric.is_better()` directly — no per-problem customization needed.

#### Associated Types

| Type | Bounds | Purpose |
|------|--------|---------|
| `Value` | `PartialOrd + Clone` | Inner objective value (e.g., `i32`, `f64`) |

#### Required Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `direction()` | `Direction` | `Direction::Maximize` or `Direction::Minimize` |

#### When to Use OptimizationProblem

**Use OptimizationProblem when**:
- Your problem has a numeric objective to optimize
- Configurations can be feasible or infeasible

**Examples that use it**:
- `MaximumIndependentSet` (maximize weight, constraint: no adjacent vertices)
- `MinimumVertexCover` (minimize weight, constraint: all edges covered)
- `MaxCut` (maximize cut weight)
- `QUBO` (minimize quadratic objective)

### 1.3 SatisfactionProblem Trait

**Location**: `src/traits.rs`

Marker trait for satisfaction (decision) problems.

```rust
pub trait SatisfactionProblem: Problem<Metric = bool> {}
```

Satisfaction problems evaluate configurations to `bool`: `true` if the configuration satisfies all constraints, `false` otherwise.

**Examples that use it**:
- `Satisfiability` (SAT)
- `KSatisfiability` (k-SAT)

### 1.4 Solver Trait

**Location**: `src/solvers/mod.rs`

Interface for all solvers.

```rust
pub trait Solver {
    /// Find one optimal solution for an optimization problem.
    fn find_best<P: OptimizationProblem>(&self, problem: &P) -> Option<Vec<usize>>;

    /// Find any satisfying solution for a satisfaction problem (Metric = bool).
    fn find_satisfying<P: Problem<Metric = bool>>(&self, problem: &P) -> Option<Vec<usize>>;
}
```

#### Contract

- `find_best()` returns one optimal configuration, or `None` if no feasible solution exists
- `find_satisfying()` returns one satisfying configuration, or `None` if unsatisfiable
- Use `BruteForce::find_all_best()` / `find_all_satisfying()` for all solutions

### 1.5 ReduceTo Trait

**Location**: `src/rules/traits.rs`

Interface for problem reductions.

```rust
pub trait ReduceTo<T: Problem>: Problem {
    /// The reduction result type.
    type Result: ReductionResult<Source = Self, Target = T>;

    /// Reduce this problem to the target problem type.
    fn reduce_to(&self) -> Self::Result;
}

pub trait ReductionResult: Clone {
    /// The source problem type.
    type Source: Problem;
    /// The target problem type.
    type Target: Problem;

    /// Get a reference to the target problem.
    fn target_problem(&self) -> &Self::Target;

    /// Extract a solution from target problem space to source problem space.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize>;
}
```

#### Usage

```rust
// Reduce source problem to target problem
let source = MaximumIndependentSet::<SimpleGraph, i32>::new(5, edges);
let reduction = source.reduce_to();  // type inferred from context
let target = reduction.target_problem();

// Solve target and extract source solution
let solver = BruteForce::new();
if let Some(target_sol) = solver.find_best(target) {
    let source_sol = reduction.extract_solution(&target_sol);
    // source_sol is now a valid solution to the original problem
}
```

---

## 2. Problem Parametrization

### 2.1 Generic Type Parameters

All graph-based problems are parametrized by:

```rust
pub struct ProblemName<G, W> {
    graph: G,
    weights: Vec<W>,
}
```

- **G**: Graph topology (e.g., `SimpleGraph`, `GridGraph`, `UnitDiskGraph`)
- **W**: Weight type (e.g., `i32`, `f64`, `One`)

#### Weight Type System

Weights use the `WeightElement` trait:

```rust
pub trait WeightElement: Clone + Default + 'static {
    /// The numeric type used for sums and comparisons.
    type Sum: NumericSize;
    /// Convert this weight element to the sum type.
    fn to_sum(&self) -> Self::Sum;
}
```

This decouples the per-element weight type from the accumulation type:
- For `i32`: `Sum = i32`, `to_sum()` returns the value
- For `f64`: `Sum = f64`, `to_sum()` returns the value
- For `One`: `Sum = i32`, `to_sum()` always returns `1`

The `NumericSize` supertrait bundles common numeric bounds:
```rust
NumericSize: Clone + Default + PartialOrd + Num + Zero + Bounded + AddAssign + 'static
```

### 2.2 Unweighted Problems

For unweighted variants, use the `One` marker type (or its alias `Unweighted`):

```rust
use problemreductions::types::One;

// Unweighted variant — all vertices have weight 1
let problem = MaximumIndependentSet::<SimpleGraph, One>::new(5, edges);
```

---

## 3. Graph Topologies

**Location**: `src/topology/`

### 3.1 Graph Trait

```rust
pub trait Graph: Clone + Send + Sync + 'static {
    /// The name of the graph type (e.g., "SimpleGraph", "GridGraph").
    const NAME: &'static str;

    fn num_vertices(&self) -> usize;
    fn num_edges(&self) -> usize;
    fn neighbors(&self, vertex: usize) -> Vec<usize>;
    fn has_edge(&self, u: usize, v: usize) -> bool;
    fn edges(&self) -> Vec<(usize, usize)>;
    // ... other methods
}
```

### 3.2 Built-in Topologies

#### SimpleGraph

**Location**: `src/topology/graph.rs`

Standard undirected simple graph (no self-loops, no multi-edges).

```rust
let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3)]);
```

#### GridGraph

**Location**: `src/topology/grid_graph.rs`

Vertices arranged in a 2D grid. Vertices are neighbors if adjacent in grid.

```rust
let graph = GridGraph::new(4, 4);  // 4x4 grid, 16 vertices total
```

#### UnitDiskGraph

**Location**: `src/topology/unit_disk_graph.rs`

Vertices in 2D Euclidean space with edges between vertices within distance 1.

```rust
let vertices = vec![(0.0, 0.0), (0.5, 0.5), (1.5, 1.5)];
let graph = UnitDiskGraph::new(vertices);
```

#### Hypergraph

**Location**: `src/topology/hypergraph.rs`

Vertices with hyperedges (edges can contain more than 2 vertices).

---

## 4. Problem Models

**Location**: `src/models/`

Problems are organized by category:

### 4.1 Graph Problems

**Location**: `src/models/graph/`

- **MaximumIndependentSet** - Maximum weight independent set (no adjacent vertices)
- **MinimumVertexCover** - Minimum weight vertex cover (cover all edges)
- **MinimumDominatingSet** - Minimum dominating set (every vertex is dominated)
- **MaxCut** - Maximum cut (maximize edges between two partitions)
- **MaximumClique** - Maximum weight clique (all vertices pairwise adjacent)
- **MaximumMatching** - Maximum weight matching (no two edges share vertices)
- **KColoring** - K-vertex coloring (adjacent vertices have different colors)
- **MaximalIS** - Maximal (not maximum) independent set
- **TravelingSalesman** - Minimum weight Hamiltonian cycle

#### Example: MaximumIndependentSet

```rust
use problemreductions::models::graph::MaximumIndependentSet;
use problemreductions::topology::SimpleGraph;

// Unit-weighted (all vertices have weight 1)
let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(5, vec![(0,1), (1,2)]);

// Custom weights
let problem = MaximumIndependentSet::<SimpleGraph, i32>::with_weights(
    5,
    vec![(0,1), (1,2)],
    vec![1, 2, 3, 4, 5]
);

// From existing graph
let graph = SimpleGraph::new(5, edges);
let problem = MaximumIndependentSet::from_graph(graph, weights);

// From graph with unit weights
let problem = MaximumIndependentSet::<SimpleGraph, i32>::from_graph_unit_weights(graph);
```

### 4.2 Satisfiability Problems

**Location**: `src/models/satisfiability/`

- **Satisfiability** - Boolean satisfiability (CNF clauses)
- **KSatisfiability** - SAT restricted to k-literal clauses

#### Example: Satisfiability

```rust
use problemreductions::models::satisfiability::{Satisfiability, CNFClause};

// Clauses in CNF: (x1 ∨ x2) ∧ (¬x2 ∨ x3) ∧ (¬x1 ∨ ¬x3)
// Literals are 1-indexed signed integers (positive = true, negative = negated)
let problem = Satisfiability::new(3, vec![
    CNFClause::new(vec![1, 2]),      // x1 ∨ x2
    CNFClause::new(vec![-2, 3]),     // ¬x2 ∨ x3
    CNFClause::new(vec![-1, -3]),    // ¬x1 ∨ ¬x3
]);
```

### 4.3 Set Problems

**Location**: `src/models/set/`

- **MinimumSetCovering** - Minimum weight set cover
- **MaximumSetPacking** - Maximum weight set packing

#### Example: MinimumSetCovering

```rust
use problemreductions::models::set::MinimumSetCovering;

// Covering problem: select sets to cover all elements
let problem = MinimumSetCovering::<i32>::new(
    3,  // universe_size (elements 0..3)
    vec![
        vec![0, 1],      // Set 0 covers elements 0, 1
        vec![1, 2],      // Set 1 covers elements 1, 2
        vec![0, 2],      // Set 2 covers elements 0, 2
    ],
);
```

### 4.4 Optimization Problems

**Location**: `src/models/optimization/`

- **SpinGlass** - Ising model Hamiltonian minimization
- **QUBO** - Quadratic unconstrained binary optimization
- **ILP** - Integer linear programming (requires `ilp` feature)

### 4.5 Specialized Problems

**Location**: `src/models/specialized/`

- **CircuitSAT** - Boolean circuit satisfiability
- **Factoring** - Integer factorization
- **PaintShop** - Minimize color switches
- **BicliqueCover** - Biclique cover on bipartite graphs
- **BMF** - Boolean matrix factorization

---

## 5. Reductions System

**Location**: `src/rules/`

### 5.1 Reduction Structure

```rust
// Define the reduction result type
pub struct ReductionSourceToTarget {
    target: TargetProblem,
    // ... solution extraction data
}

impl ReductionResult for ReductionSourceToTarget {
    type Source = SourceProblem;
    type Target = TargetProblem;

    fn target_problem(&self) -> &TargetProblem {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // Map target solution back to source variables
        // ...
    }
}

// Register the reduction
impl ReduceTo<TargetProblem> for SourceProblem {
    type Result = ReductionSourceToTarget;

    fn reduce_to(&self) -> ReductionSourceToTarget {
        // Construct target problem from source problem
        // Create solution mapping
        // ...
    }
}

// Register metadata for graph visualization
inventory::submit! {
    ReductionEntry {
        source_name: "SourceProblem",
        target_name: "TargetProblem",
        source_variant: &[("graph", "SimpleGraph"), ("weight", "i32")],
        target_variant: &[("graph", "SimpleGraph"), ("weight", "i32")],
        overhead_fn: || ReductionOverhead::new(vec![
            ("num_vars", Polynomial { ... }),
            ("num_constraints", Polynomial { ... }),
        ]),
    }
}
```

### 5.2 Built-in Reductions

The library includes reductions between these problem pairs:

- **MaximumIndependentSet ↔ MinimumVertexCover** - Complement on same graph
- **MaximumIndependentSet → MaximumSetPacking** - Element sets as independent sets
- **MaximumIndependentSet → QUBO** - Penalty encoding
- **MinimumVertexCover → MinimumSetCovering** - Elements from edges
- **MinimumVertexCover → QUBO** - Penalty encoding
- **MaximumMatching → MaximumSetPacking** - Edge representation
- **MaximumSetPacking → QUBO** - Penalty encoding
- **SAT → MaximumIndependentSet** - Variable gadgets
- **SAT → KColoring** - Clause coloring
- **SAT → MinimumDominatingSet** - Domination gadgets
- **SAT ↔ K-SAT** - Clause conversion
- **K-SAT → QUBO** - Direct QUBO encoding
- **KColoring → QUBO** - Color penalty encoding
- **SpinGlass ↔ MaxCut** - Weight transformation
- **SpinGlass ↔ QUBO** - Problem transformation
- **CircuitSAT → SpinGlass** - Logic gadgets
- **Factoring → CircuitSAT** - Multiplication circuit

Natural-edge reductions (graph subtype relaxation):
- **MIS\<Triangular\> → MIS\<SimpleGraph\>** - Identity mapping
- **MIS\<UnitDiskGraph\> → MIS\<GridGraph\>** - Graph cast
- **MIS\<SimpleGraph\> → MIS\<GridGraph\>** - Graph cast

Feature-gated reductions (require `ilp` feature):
- Various problems → **ILP** (MaximumIndependentSet, MinimumVertexCover, MaximumClique, MaximumMatching, MinimumDominatingSet, MinimumSetCovering, MaximumSetPacking, KColoring, Factoring, TravelingSalesman)
- **ILP → QUBO** - Linearization

### 5.3 Reduction Overhead

Every reduction in the library carries **overhead metadata** that describes how the target problem size relates to the source problem size. This is essential for:

1. **Cost-aware path finding**: When multiple reduction chains exist (e.g., SAT → MIS → QUBO vs SAT → 3-SAT → QUBO), the overhead lets the system pick the chain that produces the smallest target problem for a given input
2. **Documentation**: The paper and reduction graph visualization automatically display overhead formulas
3. **Planning**: Users can estimate target problem size before actually performing the reduction

#### Core Types

**Location**: `src/rules/registry.rs` and `src/polynomial.rs`

```rust
/// Overhead specification for a reduction.
pub struct ReductionOverhead {
    /// Output size as polynomials of input size variables.
    /// Each entry is (output_field_name, polynomial_formula).
    pub output_size: Vec<(&'static str, Polynomial)>,
}
```

Each entry maps an **output field name** (a dimension of the target problem) to a **polynomial** of input field names (dimensions of the source problem). For example, when reducing MIS to ILP, the output might specify that ILP's `num_vars` equals the graph's `num_vertices`, and ILP's `num_constraints` equals the graph's `num_edges`.

Polynomials are built from monomials:

```rust
/// A monomial: coefficient × Π(variable^exponent)
pub struct Monomial {
    pub coefficient: f64,
    pub variables: Vec<(&'static str, u8)>,  // (variable_name, exponent)
}

/// A polynomial: Σ monomials
pub struct Polynomial {
    pub terms: Vec<Monomial>,
}
```

#### The `poly!` Macro

Instead of constructing `Polynomial` and `Monomial` values manually, use the `poly!` convenience macro:

```rust
use problemreductions::poly;

// Single variable: p(x) = num_vertices
poly!(num_vertices)

// Variable with exponent: p(x) = num_literals²
poly!(num_literals ^ 2)

// Constant: p(x) = 5
poly!(5)

// Scaled variable: p(x) = 3 × num_vertices
poly!(3 * num_vertices)

// Scaled variable with exponent: p(x) = 9 × n²
poly!(9 * n ^ 2)

// Product of two variables: p(x) = num_vertices × num_colors
poly!(num_vertices * num_colors)

// Scaled product: p(x) = 3 × a × b
poly!(3 * a * b)

// Addition (combine with + operator):
poly!(num_vars) + poly!(num_clauses)  // p(x) = num_vars + num_clauses
```

#### Specifying Overhead in Reductions

Overhead is attached to reductions via the `#[reduction]` proc macro attribute:

```rust
#[reduction(
    overhead = {
        ReductionOverhead::new(vec![
            ("num_vertices", poly!(num_vertices)),
            ("num_edges", poly!(num_edges)),
        ])
    }
)]
impl ReduceTo<MinimumVertexCover<SimpleGraph, i32>> for MaximumIndependentSet<SimpleGraph, i32> {
    // ...
}
```

The field names (e.g., `"num_vertices"`, `"num_edges"`) in the polynomial variables refer to the **source problem's** `ProblemSize` components. The field names as keys (the first element of each tuple) name the **target problem's** size dimensions.

#### Real-World Examples

**Identity overhead** (MIS ↔ MVC complement):
```rust
// Same graph, same size — no blowup
ReductionOverhead::new(vec![
    ("num_vertices", poly!(num_vertices)),  // target has same vertices
    ("num_edges", poly!(num_edges)),        // target has same edges
])
```

**Linear overhead** (MIS → ILP):
```rust
// One ILP variable per vertex, one constraint per edge
ReductionOverhead::new(vec![
    ("num_vars", poly!(num_vertices)),
    ("num_constraints", poly!(num_edges)),
])
```

**Quadratic overhead** (SAT → MIS):
```rust
// Vertices = number of literals, edges up to literals²
ReductionOverhead::new(vec![
    ("num_vertices", poly!(num_literals)),
    ("num_edges", poly!(num_literals ^ 2)),
])
```

**Product overhead** (KColoring → QUBO):
```rust
// One QUBO variable per (vertex, color) pair
ReductionOverhead::new(vec![
    ("num_vars", poly!(num_vertices * num_colors)),
])
```

**Additive overhead** (SAT → K-SAT clause splitting):
```rust
// Splitting long clauses adds extra variables and clauses
ReductionOverhead::new(vec![
    ("num_clauses", poly!(num_clauses) + poly!(num_literals)),
    ("num_vars", poly!(num_vars) + poly!(num_literals)),
])
```

#### Evaluating Overhead at Runtime

Given a concrete input problem size, you can compute the expected target size:

```rust
let overhead = ReductionOverhead::new(vec![
    ("num_vars", poly!(num_vertices)),
    ("num_constraints", poly!(num_edges)),
]);

let input_size = ProblemSize::new(vec![("num_vertices", 100), ("num_edges", 500)]);
let output_size = overhead.evaluate_output_size(&input_size);

assert_eq!(output_size.get("num_vars"), Some(100));
assert_eq!(output_size.get("num_constraints"), Some(500));
```

### 5.4 Cost-Aware Path Finding

**Location**: `src/rules/cost.rs` and `src/rules/graph.rs`

The reduction graph supports Dijkstra-based shortest-path search with customizable cost functions. This lets you find the cheapest multi-step reduction chain between any two problem types, where "cheapest" is defined by the overhead formulas evaluated on your actual input size.

#### PathCostFn Trait

```rust
pub trait PathCostFn {
    /// Compute cost of taking an edge given current problem size.
    fn edge_cost(&self, overhead: &ReductionOverhead, current_size: &ProblemSize) -> f64;
}
```

#### Built-in Cost Functions

| Cost Function | Description | Use Case |
|--------------|-------------|----------|
| `Minimize("field")` | Minimize a single output field | "I want the fewest QUBO variables" |
| `MinimizeWeighted(vec)` | Minimize weighted sum of fields | "Balance variables and constraints" |
| `MinimizeMax(vec)` | Minimize the maximum of fields | "No single dimension should blow up" |
| `MinimizeLexicographic(vec)` | Lexicographic minimization | "Minimize vars first, break ties by constraints" |
| `MinimizeSteps` | Minimize number of reduction hops | "Shortest chain regardless of size" |
| `CustomCost(closure)` | User-defined cost from closure | Any custom objective |

#### Usage Example

```rust
use problemreductions::rules::{ReductionGraph, Minimize};
use problemreductions::types::ProblemSize;

let graph = ReductionGraph::build();

// Find cheapest path from SAT to QUBO, minimizing QUBO variables
let input_size = ProblemSize::new(vec![
    ("num_vars", 10),
    ("num_clauses", 20),
    ("num_literals", 60),  // 20 clauses × 3 literals
]);

let path = graph.find_cheapest_path(
    ("Satisfiability", ""),
    ("QUBO", ""),
    &input_size,
    &Minimize("num_vars"),
);

if let Some(path) = path {
    println!("Best chain has {} steps", path.len());
    for step in &path.edges {
        println!("  {} → {}", step.source_name, step.target_name);
    }
}
```

#### How It Works

The path finder uses Dijkstra's algorithm on the reduction graph:

```
1. Start at source node with input_size
2. For each outgoing edge, compute:
   - edge_cost = cost_fn.edge_cost(&edge.overhead, &current_size)
   - new_size  = edge.overhead.evaluate_output_size(&current_size)
3. Propagate new_size as current_size for the next hop
4. Continue until reaching the target node
5. Return the minimum-cost path
```

This means the cost function sees the **accumulated** problem size at each step, not just the original input. A chain A → B → C correctly accounts for B's intermediate size when computing the cost of B → C.

---

## 6. Solution Representation

### 6.1 Configuration Format

A configuration is `Vec<usize>` where:
- Length equals `num_variables()` (i.e., `dims().len()`)
- Each value at index `i` is in `[0, dims()[i])`

```rust
// Binary problems (dims = [2, 2, 2, 2, 2])
config[i] == 0  // Variable i NOT selected
config[i] == 1  // Variable i selected

// Multi-flavor problems (e.g., k-coloring, dims = [k, k, k, ...])
config[i] ∈ [0, k)  // Color assigned to vertex i
```

### 6.2 SolutionSize Enum

**Location**: `src/types.rs`

```rust
pub enum SolutionSize<T> {
    /// A valid (feasible) solution with the given objective value.
    Valid(T),
    /// An invalid (infeasible) solution that violates constraints.
    Invalid,
}

impl<T> SolutionSize<T> {
    pub fn is_valid(&self) -> bool;
    pub fn size(&self) -> Option<&T>;
    pub fn unwrap(self) -> T;       // panics if Invalid
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> SolutionSize<U>;
}

impl<T: PartialOrd> SolutionSize<T> {
    /// Returns true if self is a better solution than other for the given direction.
    pub fn is_better(&self, other: &Self, direction: Direction) -> bool;
}
```

**Key differences from a struct-based approach**:
- `SolutionSize::Invalid` carries no size — invalid configs simply have no meaningful objective
- Pattern matching: `if let SolutionSize::Valid(size) = result { ... }`
- `is_better()` considers: Valid always beats Invalid; two Invalids are equally bad

### 6.3 Direction Enum

**Location**: `src/types.rs`

```rust
pub enum Direction {
    Maximize,
    Minimize,
}
```

Used by `OptimizationProblem::direction()` and `SolutionSize::is_better()`.

---

## 7. Error Handling

### 7.1 Error Types

**Location**: `src/error.rs`

```rust
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ProblemError {
    #[error("invalid configuration size: expected {expected}, got {got}")]
    InvalidConfigSize { expected: usize, got: usize },

    #[error("invalid flavor value {value} at index {index}: expected 0..{num_flavors}")]
    InvalidFlavor { index: usize, value: usize, num_flavors: usize },

    #[error("invalid problem: {0}")]
    InvalidProblem(String),

    #[error("invalid weights length: expected {expected}, got {got}")]
    InvalidWeightsLength { expected: usize, got: usize },

    #[error("empty problem: {0}")]
    EmptyProblem(String),

    #[error("index out of bounds: {index} >= {bound}")]
    IndexOutOfBounds { index: usize, bound: usize },

    #[error("I/O error: {0}")]
    IoError(String),

    #[error("serialization error: {0}")]
    SerializationError(String),
}
```

### 7.2 Panic vs Result Strategy

The library uses **panics for programming errors**:

| Situation | Handling | Rationale |
|-----------|----------|-----------|
| Invalid vertex indices | Panic | Programming error |
| Weight length mismatch | Panic | Programming error |
| Invalid config length | Return `SolutionSize::Invalid` or `false` | Runtime validation |
| Constraint violation | Return `SolutionSize::Invalid` | Normal operation |

**Example**:
```rust
pub fn with_weights(..., weights: Vec<W>) -> Self {
    assert_eq!(weights.len(), num_vertices,
        "weights length must match num_vertices");
    // ...
}
```

---

## 8. Common Patterns & Workflows

### Pattern 1: Solving a Problem Directly

The simplest workflow: define a problem, then solve it.

```rust
use problemreductions::prelude::*;
use problemreductions::models::graph::MaximumIndependentSet;
use problemreductions::topology::SimpleGraph;

// Step 1: Create the problem
let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(
    5,                              // 5 vertices
    vec![(0, 1), (1, 2), (2, 3)]    // Edges
);

// Step 2: Verify problem properties
println!("Variables: {}", problem.num_variables());      // Output: 5
println!("Direction: {:?}", problem.direction());        // Maximize
println!("Variant: {:?}", MaximumIndependentSet::<SimpleGraph, i32>::variant());

// Step 3: Solve it
let solver = BruteForce::new();
let optimal_solutions = solver.find_all_best(&problem);

// Step 4: Evaluate and interpret
for solution in &optimal_solutions {
    let result = problem.evaluate(solution);
    if let SolutionSize::Valid(size) = result {
        println!("Solution: {:?}, Size: {}", solution, size);
    }
}
```

### Pattern 2: Using Reductions

Transform a problem you're interested in to another problem, solve it, extract the solution.

```rust
use problemreductions::rules::ReduceTo;

// Step 1: Create source problem
let source = MaximumIndependentSet::<SimpleGraph, i32>::new(5, edges);

// Step 2: Reduce to target problem (must implement ReduceTo)
let reduction: ReductionISToVC<_, _> = source.reduce_to();
let target = reduction.target_problem();

// Step 3: Solve the target problem
let solver = BruteForce::new();
let target_solutions = solver.find_all_best(target);

// Step 4: Extract source solutions from target solutions
for target_sol in &target_solutions {
    let source_sol = reduction.extract_solution(target_sol);
    let result = source.evaluate(&source_sol);
    if let SolutionSize::Valid(size) = result {
        println!("Original problem solution: {:?}, size: {}", source_sol, size);
    }
}
```

**Why do this?**
- Different solvers might work better for the target problem
- Reductions let you leverage algorithms designed for other problems
- Understanding reductions helps verify solution correctness

### Pattern 3: Implementing a Problem

Create a new NP-hard problem type.

```rust
use problemreductions::topology::{Graph, SimpleGraph};
use problemreductions::traits::{Problem, OptimizationProblem};
use problemreductions::types::{Direction, SolutionSize, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyProblem<G, W> {
    graph: G,
    weights: Vec<W>,
}

impl<W: Clone + Default> MyProblem<SimpleGraph, W> {
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize)>) -> Self
    where
        W: From<i32>,
    {
        let graph = SimpleGraph::new(num_vertices, edges);
        let weights = vec![W::from(1); num_vertices];
        Self { graph, weights }
    }
}

impl<G, W> Problem for MyProblem<G, W>
where
    G: Graph,
    W: WeightElement,
{
    const NAME: &'static str = "MyProblem";
    type Metric = SolutionSize<W::Sum>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", G::NAME),
            ("weight", crate::variant::short_type_name::<W>()),
        ]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_vertices()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<W::Sum> {
        // Check constraints
        // ... your constraint logic here ...

        // Compute objective
        let mut total = W::Sum::zero();
        for (i, &selected) in config.iter().enumerate() {
            if selected == 1 {
                total += self.weights[i].to_sum();
            }
        }
        SolutionSize::Valid(total)
    }
}

impl<G, W> OptimizationProblem for MyProblem<G, W>
where
    G: Graph,
    W: WeightElement,
{
    type Value = W::Sum;

    fn direction(&self) -> Direction {
        Direction::Maximize
    }
}
```

### Pattern 4: Implementing a Reduction

Transform problem A into problem B, with solution extraction.

```rust
use problemreductions::rules::{ReduceTo, ReductionResult};

// Result type holds both the transformed problem and mapping data
pub struct ReductionISToVC<G, W> {
    target: MinimumVertexCover<G, W>,
}

// Implement the result trait for extraction
impl<G, W> ReductionResult for ReductionISToVC<G, W>
where
    G: Graph,
    W: WeightElement,
{
    type Source = MaximumIndependentSet<G, W>;
    type Target = MinimumVertexCover<G, W>;

    fn target_problem(&self) -> &MinimumVertexCover<G, W> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // For complement: if target selects v, source selects NOT v
        target_solution.iter().map(|&x| 1 - x).collect()
    }
}

// Implement the reduction
impl<G, W> ReduceTo<MinimumVertexCover<G, W>> for MaximumIndependentSet<G, W>
where
    G: Graph,
    W: WeightElement,
{
    type Result = ReductionISToVC<G, W>;

    fn reduce_to(&self) -> ReductionISToVC<G, W> {
        let target = MinimumVertexCover::from_graph(
            self.graph().clone(),
            self.weights(),
        );
        ReductionISToVC { target }
    }
}
```

### Pattern 5: Solving Satisfaction Problems

```rust
use problemreductions::prelude::*;

let sat = Satisfiability::new(3, vec![
    CNFClause::new(vec![1, 2]),
    CNFClause::new(vec![-1, 3]),
]);

let solver = BruteForce::new();

// Find one satisfying assignment
if let Some(solution) = solver.find_satisfying(&sat) {
    println!("Satisfying: {:?}", solution);
}

// Find all satisfying assignments
let all = solver.find_all_satisfying(&sat);
println!("Found {} satisfying assignments", all.len());
```

---

## 9. Extension Guide

### 9.1 Adding a New Graph Problem

See [Pattern 3](#pattern-3-implementing-a-problem) above for the full implementation pattern.

Key steps:
1. Create `src/models/graph/my_problem.rs`
2. Implement `Problem` trait with `type Metric = SolutionSize<W::Sum>`
3. Implement `OptimizationProblem` with `direction()`
4. Register schema with `inventory::submit! { ProblemSchemaEntry { ... } }`
5. Register module in `src/models/graph/mod.rs`

### 9.2 Adding a Reduction

**Location**: `src/rules/<source>_<target>.rs`

See [Pattern 4](#pattern-4-implementing-a-reduction) above for the implementation pattern.

Key steps:
1. Create `src/rules/<source>_<target>.rs`
2. Define result struct implementing `ReductionResult`
3. Implement `ReduceTo` on the source problem
4. Register metadata with `inventory::submit! { ReductionEntry { ... } }`
5. Register module in `src/rules/mod.rs`

Use the `#[reduction]` proc macro attribute for automatic inventory registration:
```rust
#[reduction(
    overhead = {
        ReductionOverhead::new(vec![
            ("num_vertices", poly!(num_vertices)),
            ("num_edges", poly!(num_edges)),
        ])
    }
)]
impl ReduceTo<TargetProblem> for SourceProblem {
    // ...
}
```

### 9.3 Adding Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::BruteForce;

    #[test]
    fn test_source_to_target_closed_loop() {
        // 1. Create small instance
        let problem = SourceProblem::<SimpleGraph, i32>::new(5, vec![(0,1), (1,2)]);

        // 2. Reduce
        let reduction = problem.reduce_to();
        let target = reduction.target_problem();

        // 3. Solve both directly
        let solver = BruteForce::new();
        let source_solutions = solver.find_all_best(&problem);
        let target_solutions = solver.find_all_best(target);

        // 4. Extract and verify
        for sol in &target_solutions {
            let extracted = reduction.extract_solution(sol);
            let result = problem.evaluate(&extracted);
            assert!(result.is_valid());
        }
    }
}
```

---

## 10. Internal Modules

### 10.1 Configuration Utilities

**Location**: `src/config.rs`

```rust
/// Iterator over all configs for uniform flavor count
pub struct ConfigIterator { /* ... */ }

impl ConfigIterator {
    pub fn new(num_variables: usize, num_flavors: usize) -> Self;
    pub fn total(&self) -> usize;
}

impl Iterator for ConfigIterator {
    type Item = Vec<usize>;
}

/// Iterator over all configs for per-variable dimensions
pub struct DimsIterator { /* ... */ }

impl DimsIterator {
    pub fn new(dims: Vec<usize>) -> Self;
    pub fn total(&self) -> usize;
}

impl Iterator for DimsIterator {
    type Item = Vec<usize>;
}

/// Convert between index and configuration
pub fn index_to_config(index: usize, num_variables: usize, num_flavors: usize) -> Vec<usize>;
pub fn config_to_index(config: &[usize], num_flavors: usize) -> usize;

/// Convert between config and bits
pub fn config_to_bits(config: &[usize]) -> Vec<bool>;
pub fn bits_to_config(bits: &[bool]) -> Vec<usize>;
```

`DimsIterator` is used by `BruteForce` internally to enumerate all configurations based on `problem.dims()`.

### 10.2 ProblemSize Metadata

**Location**: `src/types.rs`

```rust
pub struct ProblemSize {
    pub components: Vec<(String, usize)>,
}

impl ProblemSize {
    pub fn new(components: Vec<(&str, usize)>) -> Self;
    pub fn get(&self, name: &str) -> Option<usize>;
}
```

### 10.3 BruteForce Solver

**Location**: `src/solvers/brute_force.rs`

```rust
#[derive(Debug, Clone, Default)]
pub struct BruteForce;

impl BruteForce {
    pub fn new() -> Self;

    /// Find all optimal solutions for an optimization problem.
    pub fn find_all_best<P: OptimizationProblem>(&self, problem: &P) -> Vec<Vec<usize>>;

    /// Find all satisfying solutions for a satisfaction problem.
    pub fn find_all_satisfying<P: Problem<Metric = bool>>(&self, problem: &P) -> Vec<Vec<usize>>;
}

impl Solver for BruteForce {
    /// Returns one optimal solution (or None).
    fn find_best<P: OptimizationProblem>(&self, problem: &P) -> Option<Vec<usize>>;

    /// Returns one satisfying solution (or None).
    fn find_satisfying<P: Problem<Metric = bool>>(&self, problem: &P) -> Option<Vec<usize>>;
}
```

### 10.4 Variant and Type Name Utilities

**Location**: `src/variant.rs`

```rust
/// Extract short type name from full path (e.g., "my_crate::types::One" → "One")
pub fn short_type_name<T: 'static>() -> &'static str;

/// Convert const generic usize to static str (for values 1-10)
pub const fn const_usize_str<const N: usize>() -> &'static str;
```

Used internally by `Problem::variant()` implementations to extract clean type names.

### 10.5 Polynomial Representation

**Location**: `src/polynomial.rs`

Used for representing reduction overhead formulas.

```rust
pub struct Polynomial {
    pub terms: Vec<PolynomialTerm>,
}

pub struct PolynomialTerm {
    pub coefficient: f64,
    pub variables: Vec<String>,
    pub exponent: f64,
}
```

### 10.6 Truth Table

**Location**: `src/truth_table.rs`

Utilities for working with boolean truth tables (used by CircuitSAT).

---

## 11. Testing Utilities

**Location**: `src/testing/`

### 11.1 Test Case Structs

```rust
/// A test case for graph problems
pub struct GraphTestCase<W> {
    pub num_vertices: usize,
    pub edges: Vec<(usize, usize)>,
    pub weights: Option<Vec<W>>,
    pub valid_solution: Vec<usize>,
    pub expected_size: W,
    pub optimal_size: Option<W>,
}

impl<W: Clone> GraphTestCase<W> {
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize)>,
               valid_solution: Vec<usize>, expected_size: W) -> Self;
    pub fn with_weights(num_vertices: usize, edges: Vec<(usize, usize)>,
                        weights: Vec<W>, valid_solution: Vec<usize>,
                        expected_size: W) -> Self;
    pub fn with_optimal(self, optimal: W) -> Self;
}

/// A test case for SAT problems
pub struct SatTestCase {
    pub num_vars: usize,
    pub clauses: Vec<Vec<i32>>,
    pub satisfying_assignment: Option<Vec<usize>>,
    pub is_satisfiable: bool,
}

impl SatTestCase {
    pub fn satisfiable(num_vars: usize, clauses: Vec<Vec<i32>>,
                       satisfying_assignment: Vec<usize>) -> Self;
    pub fn unsatisfiable(num_vars: usize, clauses: Vec<Vec<i32>>) -> Self;
}
```

### 11.2 Test Macros

```rust
// Generate comprehensive test suite for graph problems
graph_problem_tests! {
    problem_type: MaximumIndependentSet<SimpleGraph, i32>,
    test_cases: [
        (triangle, 3, [(0, 1), (1, 2), (0, 2)], [1, 0, 0], 1, true),
        (path, 3, [(0, 1), (1, 2)], [1, 0, 1], 2, true),
    ]
}

// Test that two problems are complements (e.g., IS + VC = n)
complement_test! {
    name: test_is_vc_complement,
    problem_a: MaximumIndependentSet<SimpleGraph, i32>,
    problem_b: MinimumVertexCover<SimpleGraph, i32>,
    test_graphs: [
        (3, [(0, 1), (1, 2)]),
        (4, [(0, 1), (1, 2), (2, 3)]),
    ]
}

// Quick single-instance validation
quick_problem_test!(
    MaximumIndependentSet,
    new(3, vec![(0, 1)]),
    solution: [0, 0, 1],
    expected_value: 1,
    is_max: true
);
```

---

## 12. Performance Considerations

### 12.1 Brute Force Complexity

- **Time**: O(∏ dims[i] × cost_per_evaluation)
- **Space**: O(num_variables) per configuration + result storage
- **Practical limit**: ~20 binary variables

### 12.2 Graph Representation

Uses adjacency list internally (petgraph):

- **Neighbor lookup**: O(degree)
- **Memory**: O(V + E)
- **Best for**: Sparse graphs

### 12.3 Monomorphization

Use concrete types to avoid dynamic dispatch:

```rust
// Good: Monomorphized, fast
MaximumIndependentSet::<SimpleGraph, i32>::new(...)
MaximumIndependentSet::<SimpleGraph, f64>::new(...)

// Also good: Generic function, monomorphized per call site
fn solve<P: OptimizationProblem>(p: &P) -> Option<Vec<usize>> {
    BruteForce::new().find_best(p)
}
```

---

## 13. Known Limitations

| Limitation | Implication | Workaround |
| --- | --- | --- |
| Vertex indices must be 0..n | No automatic remapping | Preprocess input |
| BruteForce solver is O(2^n) | Impractical for large instances | Use ILP solver (enabled by default in CLI, feature-gated in library) |
| No parallel evaluation | Single-threaded | Future: parallel config iteration |

---

## Quick Reference

### Creating Problems

```rust
// Unit-weighted graph problem
let is = MaximumIndependentSet::<SimpleGraph, i32>::new(5, vec![(0,1), (1,2)]);

// Weighted graph problem
let is = MaximumIndependentSet::<SimpleGraph, i32>::with_weights(5, edges, vec![1,2,3,4,5]);

// SAT problem
let sat = Satisfiability::new(3, vec![
    CNFClause::new(vec![1, 2]),
    CNFClause::new(vec![-1, 3]),
    CNFClause::new(vec![-2, -3]),
]);
```

### Evaluating Solutions

```rust
let config = vec![1, 0, 1, 0, 1];
let result = problem.evaluate(&config);
match result {
    SolutionSize::Valid(size) => println!("Size: {}", size),
    SolutionSize::Invalid => println!("Infeasible"),
}
```

### Solving

```rust
let solver = BruteForce::new();

// One optimal solution
if let Some(sol) = solver.find_best(&problem) {
    println!("{:?}", sol);
}

// All optimal solutions
let all = solver.find_all_best(&problem);
```

### Using Reductions

```rust
let source = MaximumIndependentSet::<SimpleGraph, i32>::new(5, edges);
let reduction = source.reduce_to();
let target = reduction.target_problem();

let solver = BruteForce::new();
if let Some(target_sol) = solver.find_best(target) {
    let source_sol = reduction.extract_solution(&target_sol);
    println!("Source solution: {:?}", source_sol);
}
```

### Getting Problem Information

```rust
let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![(0,1), (1,2)]);
println!("Name: {}", MaximumIndependentSet::<SimpleGraph, i32>::NAME);
println!("Variant: {:?}", MaximumIndependentSet::<SimpleGraph, i32>::variant());
println!("Variables: {}", problem.num_variables());
println!("Dims: {:?}", problem.dims());
println!("Direction: {:?}", problem.direction());
```

### CLI Quick Reference

```bash
# Explore
pred list                                          # all problem types
pred show MIS                                      # problem details
pred path MIS QUBO                                 # find reduction path

# Create → Solve (one-liner)
pred create MIS --edges 0-1,1-2,2-3 | pred solve -

# Create → Reduce → Solve
pred create MIS --edges 0-1,1-2,2-3 -o p.json
pred reduce p.json --to QUBO -o bundle.json
pred solve bundle.json

# Evaluate a configuration
pred evaluate p.json --config 1,0,1,0
```

## 14. FAQ & Troubleshooting

### Common Questions

#### Q: How do I choose between MaximumIndependentSet, MinimumVertexCover, and other graph problems?

**A:** They're often equivalent via reductions:

```text
Independent Set: Select vertices with no edges between them
  - Goal: MAXIMIZE weight
  - Constraint: No two selected vertices are adjacent

Vertex Cover: Select vertices that touch all edges
  - Goal: MINIMIZE weight
  - Constraint: Every edge has at least one endpoint selected

Clique: Select vertices that are ALL adjacent to each other
  - Goal: MAXIMIZE weight
  - Constraint: Every pair of selected vertices is adjacent

Key insight: Their solutions are complements!
  IS_solution = [1, 0, 1, 0]  (vertices 0, 2 selected)
  VC_solution = [0, 1, 0, 1]  (vertices 1, 3 selected)
  IS + VC = [1, 1, 1, 1]      (all vertices)
```

**Choose based on your application**:

- Independent Set: "find non-interfering items" (assignments, scheduling)
- Vertex Cover: "find minimum monitors" (surveillance, redundancy)
- Clique: "find communities" (social networks, dense subgraphs)

#### Q: What's the difference between One and i32 weight type?

**A:**

```rust
// Both create problems with all weights = 1
MaximumIndependentSet::<SimpleGraph, One>::new(5, edges)
MaximumIndependentSet::<SimpleGraph, i32>::new(5, edges)  // defaults to weight 1

// Difference is semantic and in the variant:
One: "This problem is inherently unweighted, all vertices equal"
     variant() reports ("weight", "One")
i32: "This problem can have any integer weights"
     variant() reports ("weight", "i32")

// You can change weights in i32 version:
MaximumIndependentSet::<SimpleGraph, i32>::with_weights(5, edges, vec![1,2,3,4,5])
```

**Use One when**:
- Problem is theoretically unweighted (pure complexity question)

**Use i32 when**:
- Problem naturally has weights (generalization)
- You need flexibility

#### Q: Why does reducing MaximumIndependentSet to MinimumVertexCover give a larger problem?

**A:** In this case, it doesn't! It's a complement:

```rust
// Same graph, same vertices
let is = MaximumIndependentSet::<SimpleGraph, i32>::new(5, edges);
let reduction: ReductionISToVC<_, _> = is.reduce_to();

// The reduced problem has the SAME structure
// We just flip 0→1 and 1→0 in the solution extraction
```

But in other reductions, overhead is real:

```text
SAT (3 variables, 5 clauses) → MaximumIndependentSet

Source: 3 variables, 5 clauses
Target: ~30 variables! (3 + 5*5)
        Why? Each clause needs gadget vertices

This is why some problems are "harder to solve":
- Direct brute force on SAT: 2^3 = 8 configurations
- After reduction: 2^30 = 1 billion configurations
```

#### Q: How do I know if a reduction is correct?

**A:** Every reduction must pass the **closed-loop test**:

```rust
#[test]
fn test_reduction_closed_loop() {
    // 1. Create source problem
    let source = SourceProblem::new(...);

    // 2. Get optimal solutions to source (direct)
    let solver = BruteForce::new();
    let source_solutions = solver.find_all_best(&source);

    // 3. Reduce and solve target
    let reduction = source.reduce_to();
    let target = reduction.target_problem();
    let target_solutions = solver.find_all_best(target);

    // 4. Extract and verify: target solutions map to valid source solutions
    for target_sol in &target_solutions {
        let extracted = reduction.extract_solution(target_sol);
        let source_result = source.evaluate(&extracted);
        assert!(source_result.is_valid());
    }

    // 5. Both should have same optimal value
    // This proves the reduction preserves optimality
}
```

#### Q: Can I implement my own solver?

**A:** Yes! Implement the `Solver` trait:

```rust
use problemreductions::solvers::Solver;
use problemreductions::traits::{OptimizationProblem, Problem};

pub struct MyHeuristic {
    max_iterations: usize,
}

impl Solver for MyHeuristic {
    fn find_best<P: OptimizationProblem>(&self, problem: &P) -> Option<Vec<usize>> {
        // Your optimization algorithm here
        None  // Placeholder
    }

    fn find_satisfying<P: Problem<Metric = bool>>(&self, problem: &P) -> Option<Vec<usize>> {
        // Your satisfaction algorithm here
        None  // Placeholder
    }
}
```

#### Q: What if I only want to work with certain graph types?

**A:** You can constrain the generic parameter:

```rust
// This function works with ANY graph topology
fn count_variables<G: Graph, W: WeightElement>(
    problem: &MaximumIndependentSet<G, W>
) -> usize {
    problem.num_variables()
}

// This function works ONLY with SimpleGraph
fn my_special_solver(
    problem: &MaximumIndependentSet<SimpleGraph, i32>
) -> Option<Vec<usize>> {
    // Can access SimpleGraph-specific features here
    let graph = problem.graph();
    // ...
    None
}
```

#### Q: How do I serialize/deserialize problems?

**A:** All problems implement `Serialize` and `Deserialize`:

```rust
use serde_json;

let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(5, edges);

// To JSON
let json = serde_json::to_string(&problem).unwrap();

// From JSON (if you know the type)
let loaded: MaximumIndependentSet<SimpleGraph, i32> =
    serde_json::from_str(&json).unwrap();
```

**Limitation**: Type information is lost. You must know the problem type when deserializing.

---

### Debugging Tips

#### Problem: "Variable out of bounds" panic

**Cause**: You're creating edges or weights with invalid vertex indices.

```rust
// WRONG: Vertex 5 doesn't exist (only 0-4)
let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(5, vec![(0, 5)]);

// CORRECT: Valid vertex indices
let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(5, vec![(0, 4)]);
```

#### Problem: "Weights length must match" panic

**Cause**: Weights vector has wrong length.

```rust
let vertices = 5;
let edges = vec![(0, 1)];

// WRONG: 3 weights for 5 vertices
let p = MaximumIndependentSet::with_weights(vertices, edges, vec![1, 2, 3]);

// CORRECT: 5 weights for 5 vertices
let p = MaximumIndependentSet::with_weights(vertices, edges, vec![1, 2, 3, 4, 5]);
```

#### Problem: Configuration returns Invalid even though it looks correct

**Cause**: Likely constraint violation, not configuration validity.

```rust
let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(
    3,
    vec![(0, 1), (1, 2)]  // Path: 0-1-2
);

// INVALID: Both 0 and 1 selected, but edge (0,1) forbids this
let config = vec![1, 1, 0];
let result = problem.evaluate(&config);
assert!(!result.is_valid());  // SolutionSize::Invalid

// VALID: Vertices 0 and 2 are not adjacent
let config = vec![1, 0, 1];
let result = problem.evaluate(&config);
assert!(result.is_valid());   // SolutionSize::Valid(2)
```

#### Problem: Reduction seems to lose solutions

**Cause**: Check if your solution extraction is correct.

```rust
// In your ReductionResult implementation:
impl ReductionResult for MyReduction {
    type Source = SourceProblem;
    type Target = TargetProblem;

    fn extract_solution(&self, target_sol: &[usize]) -> Vec<usize> {
        // WRONG: Just return target solution unchanged
        target_sol.to_vec()  // Wrong for complement!

        // CORRECT: Apply proper transformation
        target_sol.iter().map(|&x| 1 - x).collect()  // Flip bits for complement
    }
}
```

---

## 15. Complete End-to-End Example

Here's a comprehensive example showing how all the pieces work together:

```rust
use problemreductions::prelude::*;
use problemreductions::models::graph::MaximumIndependentSet;
use problemreductions::topology::SimpleGraph;
use problemreductions::solvers::BruteForce;

fn main() {
    // ============ STEP 1: Create the problem ============
    // We have a simple graph with 4 vertices and 3 edges
    let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(
        4,
        vec![(0, 1), (1, 2), (2, 3)]  // Linear chain: 0-1-2-3
    );

    println!("=== Problem Definition ===");
    println!("Name: {}", MaximumIndependentSet::<SimpleGraph, i32>::NAME);
    println!("Variant: {:?}", MaximumIndependentSet::<SimpleGraph, i32>::variant());
    println!("Variables: {}", problem.num_variables());
    println!("Dims: {:?}", problem.dims());
    println!("Direction: {:?}", problem.direction());

    // ============ STEP 2: Manually test some configurations ============
    println!("\n=== Manual Configuration Tests ===");

    // Configuration 1: Select only vertex 0
    let config1 = vec![1, 0, 0, 0];
    let result1 = problem.evaluate(&config1);
    println!("Config [1,0,0,0]: {:?}", result1);  // Valid(1)

    // Configuration 2: Select vertices 0 and 2 (non-adjacent)
    let config2 = vec![1, 0, 1, 0];
    let result2 = problem.evaluate(&config2);
    println!("Config [1,0,1,0]: {:?}", result2);  // Valid(2)

    // Configuration 3: Select vertices 0 and 1 (adjacent - invalid!)
    let config3 = vec![1, 1, 0, 0];
    let result3 = problem.evaluate(&config3);
    println!("Config [1,1,0,0]: {:?}", result3);  // Invalid

    // ============ STEP 3: Solve optimally using BruteForce ============
    println!("\n=== Solving with BruteForce ===");
    let solver = BruteForce::new();
    let optimal_solutions = solver.find_all_best(&problem);

    println!("Found {} optimal solutions:", optimal_solutions.len());
    for (i, solution) in optimal_solutions.iter().enumerate() {
        let result = problem.evaluate(solution);
        println!("  Solution {}: {:?} ({:?})", i, solution, result);
    }

    // ============ STEP 4: Use reduction to solve via another problem ============
    println!("\n=== Solving via Reduction ===");
    let reduction: ReductionISToVC<_, _> = problem.reduce_to();
    let target_problem = reduction.target_problem();

    println!("Reduced to MinimumVertexCover");
    println!("MVC variables: {}", target_problem.num_variables());
    println!("MVC direction: {:?}", target_problem.direction());

    // Solve the target problem
    let target_solutions = solver.find_all_best(target_problem);
    println!("Found {} MVC solutions", target_solutions.len());

    // Extract back to original problem
    for (i, target_sol) in target_solutions.iter().enumerate() {
        let extracted = reduction.extract_solution(target_sol);
        let result = problem.evaluate(&extracted);

        println!("  MVC solution {}: {:?} → IS: {:?} ({:?})",
                 i, target_sol, extracted, result);
    }

    // ============ STEP 5: Verify consistency ============
    println!("\n=== Verification ===");
    let direct_size = optimal_solutions.iter()
        .filter_map(|sol| problem.evaluate(sol).size().copied())
        .max()
        .unwrap_or(0);

    let via_reduction_size = target_solutions.iter()
        .filter_map(|target_sol| {
            let extracted = reduction.extract_solution(target_sol);
            problem.evaluate(&extracted).size().copied()
        })
        .max()
        .unwrap_or(0);

    println!("Direct IS solve: {} vertices selected", direct_size);
    println!("Via MVC reduction: {} vertices selected", via_reduction_size);
    println!("Consistent: {}", direct_size == via_reduction_size);
}
```

### Understanding the Example

1. **Problem Creation**: We create a MaximumIndependentSet problem on a 4-vertex path graph
2. **Manual Testing**: We evaluate three configurations to understand constraints
3. **Direct Solve**: Use BruteForce to find all optimal solutions
4. **Reduction**: Transform to MinimumVertexCover and solve via reduction
5. **Verification**: Check that both approaches agree

**Key insights**:

- MIS and MVC are **complement problems** on the same graph
- MIS **maximizes** (Direction::Maximize) selection
- MVC **minimizes** (Direction::Minimize) selection
- Solutions are related: IS_sol = NOT(VC_sol)
- Both methods find the same optimal value!

---

## 16. CLI Tool (`pred`)

The `pred` CLI tool provides a command-line interface for exploring NP-hard problem reductions without writing Rust code. It's published as a separate crate (`problemreductions-cli`, binary name `pred`).

### 16.1 Installation

```bash
# From crates.io (default: HiGHS ILP backend)
cargo install problemreductions-cli

# With alternative ILP backends
cargo install problemreductions-cli --features coin-cbc
cargo install problemreductions-cli --features scip
cargo install problemreductions-cli --no-default-features --features clarabel

# From source
make cli    # builds target/release/pred
```

### 16.2 Global Flags

All commands support these flags:

| Flag | Description |
| --- | --- |
| `-o, --output <PATH>` | Save output as JSON to file (implies JSON mode) |
| `--json` | Output JSON to stdout instead of human-readable text |
| `-q, --quiet` | Suppress informational messages on stderr |

### 16.3 Problem Aliases

Short aliases are supported everywhere a problem name is expected:

| Alias | Full Name |
| --- | --- |
| `MIS` | MaximumIndependentSet |
| `MVC` | MinimumVertexCover |
| `SAT` | Satisfiability |
| `3SAT` | KSatisfiability (K=3) |
| `KSAT` | KSatisfiability |
| `TSP` | TravelingSalesman |

Unknown names trigger fuzzy-match suggestions.

### 16.4 Graph Exploration

#### `pred list` — List all problem types

```bash
pred list                   # human-readable table
pred list --json | jq '.'   # JSON output
```

#### `pred show <PROBLEM>` — Problem details

```bash
pred show MIS                   # variants, fields, reductions
pred show MIS/UnitDiskGraph     # specific graph variant
```

#### `pred to` / `pred from` — Explore reduction neighbors

```bash
pred to MIS              # 1-hop: what MIS reduces to
pred to MIS --hops 2     # 2-hop reachable targets
pred from QUBO           # what reduces to QUBO
pred from QUBO --hops 3
```

Output is an ASCII tree visualization.

#### `pred path <SOURCE> <TARGET>` — Find reduction paths

```bash
pred path MIS QUBO                              # cheapest path
pred path MIS QUBO --all                        # all paths
pred path MIS QUBO -o path.json                 # save for use with `pred reduce --via`
pred path MIS QUBO --cost minimize:num_variables # custom cost function
```

Cost functions:
- `minimize-steps` (default) — fewest reduction hops
- `minimize:<field>` — minimize a specific target size field (e.g., `num_variables`)

#### `pred export-graph` — Export full reduction graph

```bash
pred export-graph -o reduction_graph.json
```

### 16.5 Creating Problem Instances

#### `pred create <PROBLEM> [OPTIONS]`

**Graph problems** (MIS, MVC, MaxCut, MaxClique, MaximumMatching, MinimumDominatingSet, SpinGlass, TSP):

```bash
pred create MIS --edges 0-1,1-2,2-3 -o problem.json
pred create MIS --edges 0-1,1-2 --weights 2,1,3 -o weighted.json
```

**SAT problems**:

```bash
pred create SAT --num-vars 3 --clauses "1,2;-1,3" -o sat.json
pred create 3SAT --num-vars 4 --clauses "1,2,3;-1,2,-3" -o 3sat.json
```

**QUBO**:

```bash
pred create QUBO --matrix "1,0.5;0.5,2" -o qubo.json
```

**KColoring**:

```bash
pred create KColoring --k 3 --edges 0-1,1-2,2-0 -o kcol.json
```

**Factoring**:

```bash
pred create Factoring --target 15 --bits-m 4 --bits-n 4 -o factoring.json
```

**Random graph generation**:

```bash
pred create MIS --random --num-vertices 10 --edge-prob 0.3
pred create MIS --random --num-vertices 10 --seed 42 -o big.json
```

### 16.6 Evaluating and Inspecting

#### `pred evaluate <FILE> --config <CONFIG>`

```bash
pred evaluate problem.json --config 1,0,1,0
pred evaluate problem.json --config 1,0,1,0 -o result.json
```

#### `pred inspect <FILE>`

Shows problem type, size metrics, available solvers, and reduction targets.

```bash
pred inspect problem.json
pred inspect bundle.json
```

### 16.7 Solving

#### `pred solve <FILE> [OPTIONS]`

```bash
pred solve problem.json                        # ILP solver (default, auto-reduces)
pred solve problem.json --solver brute-force   # exhaustive search
pred solve problem.json --timeout 10           # abort after 10 seconds
pred solve bundle.json                         # solve a reduction bundle
```

The ILP solver auto-reduces non-ILP problems to ILP before solving. When given a reduction bundle (from `pred reduce`), it solves the target and maps the solution back.

### 16.8 Reducing

#### `pred reduce <FILE> --to <TARGET> [OPTIONS]`

```bash
pred reduce problem.json --to QUBO -o reduced.json
pred reduce problem.json --to ILP -o reduced.json
pred reduce problem.json --via path.json -o reduced.json   # explicit route
```

Output is a reduction bundle containing source, target, and path metadata. Feed it to `pred solve` to solve and extract the original solution.

### 16.9 Piping and Stdin

All file-accepting commands support `-` for stdin:

```bash
pred create MIS --edges 0-1,1-2 | pred solve -
pred create MIS --edges 0-1,1-2 | pred evaluate - --config 1,0,1
pred create MIS --edges 0-1,1-2 | pred reduce - --to QUBO | pred solve -
pred create MIS --edges 0-1,1-2 | pred inspect -
```

### 16.10 Shell Completions

```bash
# Auto-detect shell
eval "$(pred completions)"

# Or specify: bash, zsh, fish
eval "$(pred completions zsh)"
```

### 16.11 End-to-End CLI Workflow

```bash
# 1. Explore the reduction graph
pred show MIS
pred path MIS QUBO

# 2. Create a problem instance
pred create MIS --edges 0-1,1-2,2-3,3-4,4-0 -o problem.json

# 3. Solve directly (auto-reduces to ILP)
pred solve problem.json -o solution.json

# 4. Or: explicit reduction + solve
pred reduce problem.json --to QUBO -o bundle.json
pred solve bundle.json --solver brute-force

# 5. Verify a known configuration
pred evaluate problem.json --config 1,0,1,0,0
```

### 16.12 JSON Output Formats

All commands support `--json` or `-o` for structured output:

- **Problem JSON**: `{"type": "...", "variant": {...}, "data": {...}}`
- **Reduction bundle**: `{"source": {...}, "target": {...}, "path": [...]}`
- **Solution JSON**: `{"problem": "...", "solver": "...", "solution": [...], "evaluation": "..."}`

---
