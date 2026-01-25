# Set Problems

## SetCovering

Find minimum weight collection of sets that covers all elements.

```rust
use problemreductions::prelude::*;

let problem = SetCovering::<i32>::new(
    5,  // Universe size
    vec![
        vec![0, 1, 2],
        vec![2, 3, 4],
        vec![0, 4],
    ],
);
```

## SetPacking

Find maximum weight collection of pairwise disjoint sets.

```rust
use problemreductions::prelude::*;

let problem = SetPacking::<i32>::new(vec![
    vec![0, 1],
    vec![1, 2],  // Overlaps with first
    vec![2, 3],
    vec![4],
]);
```

## Relationship to Graph Problems

SetPacking is equivalent to IndependentSet on the intersection graph:
- Each set becomes a vertex
- Overlapping sets are connected by edges

The library provides reductions between them.
