# Available Reductions

## IndependentSet ↔ VertexCovering

These are complement problems on the same graph.

```rust
use problemreductions::prelude::*;

let is = IndependentSet::<i32>::new(4, vec![(0, 1), (1, 2)]);
let result = ReduceTo::<VertexCovering<i32>>::reduce_to(&is);
```

For any graph: `|max IS| + |min VC| = n`

## IndependentSet ↔ SetPacking

Based on the intersection graph equivalence.

```rust
use problemreductions::prelude::*;

let is = IndependentSet::<i32>::new(3, vec![(0, 1), (1, 2)]);
let result = ReduceTo::<SetPacking<i32>>::reduce_to(&is);
```

## SpinGlass ↔ QUBO

Uses variable substitution `s = 2x - 1`.

```rust
use problemreductions::prelude::*;

let sg = SpinGlass::new(2, vec![((0, 1), -1.0)], vec![0.0, 0.0]);
let result = ReduceTo::<QUBO>::reduce_to(&sg);
```

## SpinGlass ↔ MaxCut

Direct mapping for pure interaction terms; ancilla vertex for on-site fields.

```rust
use problemreductions::prelude::*;

let sg = SpinGlass::new(3, vec![((0, 1), 1), ((1, 2), 1)], vec![0, 0, 0]);
let result = ReduceTo::<MaxCut<i32>>::reduce_to(&sg);
```
