# Available Reductions

This page documents all reduction rules implemented in the library, with academic citations.

## Graph Problem Reductions

### IndependentSet <-> VertexCovering

These are complement problems on the same graph. A set S is an independent set if and only if V \ S is a vertex cover.

**Citation**: Karp, R. M. (1972). "Reducibility among Combinatorial Problems". In Miller, R. E.; Thatcher, J. W. (eds.). Complexity of Computer Computations. Plenum Press. pp. 85-103.

```rust
use problemreductions::prelude::*;

let is = IndependentSet::<i32>::new(4, vec![(0, 1), (1, 2)]);
let result = ReduceTo::<VertexCovering<i32>>::reduce_to(&is);
```

For any graph: `|max IS| + |min VC| = n`

### IndependentSet <-> SetPacking

Based on the intersection graph equivalence. Each vertex becomes a set containing its incident edges.

**Citation**: Gavril, F. (1972). "Algorithms for Minimum Coloring, Maximum Clique, Minimum Covering by Cliques, and Maximum Independent Set of a Chordal Graph". SIAM Journal on Computing. 1 (2): 180-187.

```rust
use problemreductions::prelude::*;

let is = IndependentSet::<i32>::new(3, vec![(0, 1), (1, 2)]);
let result = ReduceTo::<SetPacking<i32>>::reduce_to(&is);
```

### VertexCovering -> SetCovering

Each vertex becomes a set containing the edges it covers. The universe is the set of all edges.

**Citation**: Karp, R. M. (1972). "Reducibility among Combinatorial Problems". In Miller, R. E.; Thatcher, J. W. (eds.). Complexity of Computer Computations. Plenum Press. pp. 85-103.

```rust
use problemreductions::prelude::*;

let vc = VertexCovering::<i32>::new(3, vec![(0, 1), (1, 2)]);
let result = ReduceTo::<SetCovering<i32>>::reduce_to(&vc);
```

### Matching -> SetPacking

Each edge (u, v) becomes a set {u, v}. A matching (no two edges share a vertex) corresponds to a set packing (no two sets overlap).

**Citation**: Edmonds, J. (1965). "Paths, Trees, and Flowers". Canadian Journal of Mathematics. 17: 449-467.

```rust
use problemreductions::prelude::*;

let matching = Matching::<i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
let result = ReduceTo::<SetPacking<i32>>::reduce_to(&matching);
```

## Optimization Reductions

### SpinGlass <-> QUBO

Uses variable substitution `s = 2x - 1` to convert between spin variables (±1) and binary variables (0/1).

**Citation**: Lucas, A. (2014). "Ising formulations of many NP problems". Frontiers in Physics. 2: 5.

```rust
use problemreductions::prelude::*;

let sg = SpinGlass::new(2, vec![((0, 1), -1.0)], vec![0.0, 0.0]);
let result = ReduceTo::<QUBO>::reduce_to(&sg);
```

### SpinGlass <-> MaxCut

Direct mapping for pure interaction terms. On-site fields require an ancilla vertex connected to all spins.

**Citation**: Barahona, F. (1982). "On the computational complexity of Ising spin glass models". Journal of Physics A: Mathematical and General. 15 (10): 3241-3253.

```rust
use problemreductions::prelude::*;

let sg = SpinGlass::new(3, vec![((0, 1), 1), ((1, 2), 1)], vec![0, 0, 0]);
let result = ReduceTo::<MaxCut<i32>>::reduce_to(&sg);
```

## SAT-Based Reductions

### Satisfiability <-> KSatisfiability

Converts between general CNF-SAT and k-SAT (clauses with exactly k literals). Large clauses are split using auxiliary variables; small clauses are padded.

**Citation**: Cook, S. A. (1971). "The Complexity of Theorem-Proving Procedures". Proceedings of the Third Annual ACM Symposium on Theory of Computing. pp. 151-158.

```rust
use problemreductions::prelude::*;

let sat = Satisfiability::<i32>::new(3, vec![
    CNFClause::new(vec![1, 2, 3]),
    CNFClause::new(vec![-1, 2]),
]);
let result = ReduceTo::<KSatisfiability<3, i32>>::reduce_to(&sat);
```

### Satisfiability -> IndependentSet

Each literal occurrence in a clause becomes a vertex. Edges connect conflicting literals (x and NOT x) and literals within the same clause.

**Citation**: Karp, R. M. (1972). "Reducibility among Combinatorial Problems". In Miller, R. E.; Thatcher, J. W. (eds.). Complexity of Computer Computations. Plenum Press. pp. 85-103.

```rust
use problemreductions::prelude::*;

let sat = Satisfiability::<i32>::new(2, vec![
    CNFClause::new(vec![1, 2]),
    CNFClause::new(vec![-1, -2]),
]);
let result = ReduceTo::<IndependentSet<i32>>::reduce_to(&sat);
```

### Satisfiability -> Coloring

Uses gadget-based construction with three special colors (TRUE, FALSE, AUX). Variables are represented as vertex pairs, and OR-gadgets enforce clause satisfaction.

**Citation**: Garey, M. R.; Johnson, D. S. (1979). Computers and Intractability: A Guide to the Theory of NP-Completeness. W. H. Freeman. ISBN 0-7167-1045-5.

```rust
use problemreductions::prelude::*;

let sat = Satisfiability::<i32>::new(2, vec![
    CNFClause::new(vec![1, 2]),
]);
let result = ReduceTo::<Coloring>::reduce_to(&sat);
```

### Satisfiability -> DominatingSet

Variables and clauses become vertices. A variable vertex dominates clause vertices containing that literal.

**Citation**: Garey, M. R.; Johnson, D. S. (1979). Computers and Intractability: A Guide to the Theory of NP-Completeness. W. H. Freeman. ISBN 0-7167-1045-5.

```rust
use problemreductions::prelude::*;

let sat = Satisfiability::<i32>::new(2, vec![
    CNFClause::new(vec![1, -2]),
]);
let result = ReduceTo::<DominatingSet<i32>>::reduce_to(&sat);
```

## Circuit Reductions

### CircuitSAT -> SpinGlass

Each logic gate is mapped to a spin glass gadget that encodes the gate's truth table as energy penalties.

**Citation**: Whitfield, J. D.; Faccin, M.; Biamonte, J. D. (2012). "Ground-state spin logic". EPL (Europhysics Letters). 99 (5): 57004.

```rust
use problemreductions::prelude::*;

let circuit = Circuit::new(vec![
    Assignment::new(vec!["c".to_string()],
        BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")])),
]);
let problem = CircuitSAT::<i32>::new(circuit);
let result = ReduceTo::<SpinGlass<i32>>::reduce_to(&problem);
```

### Factoring -> CircuitSAT

Constructs a multiplier circuit that computes p * q and constrains the output to equal the target number N. A satisfying assignment reveals the factors.

**Citation**: Folklore result using standard array multiplier circuits. The construction is well-known in hardware verification and circuit complexity.

```rust
use problemreductions::prelude::*;

// Factor 15 with 4-bit factors
let factoring = Factoring::new(4, 4, 15);
let result = ReduceTo::<CircuitSAT<i32>>::reduce_to(&factoring);
```

## Reduction Summary Table

| Source | Target | Type | Reference |
|--------|--------|------|-----------|
| IndependentSet | VertexCovering | Complement | Karp 1972 |
| VertexCovering | IndependentSet | Complement | Karp 1972 |
| IndependentSet | SetPacking | Intersection graph | Gavril 1972 |
| SetPacking | IndependentSet | Intersection graph | Gavril 1972 |
| VertexCovering | SetCovering | Edge covering | Karp 1972 |
| Matching | SetPacking | Edge-to-set | Edmonds 1965 |
| SpinGlass | QUBO | Variable substitution | Lucas 2014 |
| QUBO | SpinGlass | Variable substitution | Lucas 2014 |
| SpinGlass | MaxCut | Direct mapping | Barahona 1982 |
| MaxCut | SpinGlass | Direct mapping | Barahona 1982 |
| Satisfiability | KSatisfiability | Clause splitting | Cook 1971 |
| KSatisfiability | Satisfiability | Direct embedding | Cook 1971 |
| Satisfiability | IndependentSet | Gadget | Karp 1972 |
| Satisfiability | Coloring | Gadget | Garey & Johnson 1979 |
| Satisfiability | DominatingSet | Gadget | Garey & Johnson 1979 |
| CircuitSAT | SpinGlass | Gate gadgets | Whitfield et al. 2012 |
| Factoring | CircuitSAT | Multiplier circuit | Folklore |
