# Graph Problems

## IndependentSet

Find a maximum weight set of vertices where no two are adjacent.

```rust
use problemreductions::prelude::*;

let problem = IndependentSet::<i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
let solver = BruteForce::new();
let solutions = solver.find_best(&problem);
// Maximum IS on a path of 4 vertices has size 2
```

## VertexCovering

Find a minimum weight set of vertices that covers all edges.

```rust
use problemreductions::prelude::*;

let problem = VertexCovering::<i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
// Minimum vertex cover on a path of 4 vertices has size 2
```

**Relationship**: IS and VC are complements. For any graph: `|max IS| + |min VC| = n`.

## MaxCut

Partition vertices into two sets to maximize the weight of edges between them.

```rust
use problemreductions::prelude::*;

let problem = MaxCut::new(3, vec![(0, 1, 2), (1, 2, 3), (0, 2, 1)]);
```

## Coloring

Assign k colors to vertices minimizing adjacent same-color pairs.

```rust
use problemreductions::prelude::*;

let problem = Coloring::new(4, 3, vec![(0, 1), (1, 2), (2, 3)]);
```

## DominatingSet

Find minimum set where every vertex is in the set or adjacent to it.

## MaximalIS

Find independent sets that cannot be extended.

## Matching

Find maximum weight set of non-adjacent edges.
