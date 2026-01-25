# Specialized Problems

## PaintShop

Minimize color switches in a painting sequence where each car type needs both colors.

```rust
use problemreductions::prelude::*;

let problem = PaintShop::new(vec!["a", "b", "a", "b", "c", "c"]);
```

## BicliqueCover

Cover edges of a bipartite graph using k bicliques.

```rust
use problemreductions::prelude::*;

let problem = BicliqueCover::new(
    2,  // Left vertices
    2,  // Right vertices
    vec![(0, 2), (0, 3), (1, 2), (1, 3)],  // Edges
    1,  // Number of bicliques
);
```

## BMF (Boolean Matrix Factorization)

Factor a boolean matrix M into A * B with minimum rank.

```rust
use problemreductions::prelude::*;

let problem = BMF::new(
    vec![
        vec![true, true],
        vec![true, false],
    ],
    1,  // Rank
);
```
