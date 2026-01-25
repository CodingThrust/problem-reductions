# Optimization Problems

## SpinGlass (Ising Model)

Minimize the Ising Hamiltonian: `H = Σ J_ij s_i s_j + Σ h_i s_i`

```rust
use problemreductions::prelude::*;

let problem = SpinGlass::new(
    3,
    vec![((0, 1), -1.0), ((1, 2), 1.0)],  // Interactions
    vec![0.5, -0.5, 0.0],                  // On-site fields
);

let solver = BruteForce::new();
let solutions = solver.find_best(&problem);
```

## QUBO

Quadratic Unconstrained Binary Optimization: `minimize x^T Q x`

```rust
use problemreductions::prelude::*;

// From matrix
let problem = QUBO::from_matrix(vec![
    vec![1.0, -2.0],
    vec![0.0, 1.0],
]);

// From linear and quadratic terms
let problem = QUBO::new(
    vec![1.0, -1.0],           // Linear (diagonal)
    vec![((0, 1), 0.5)],       // Quadratic (off-diagonal)
);
```

## Relationship

SpinGlass and QUBO are equivalent via the transformation `s = 2x - 1`:
- `x = 0` corresponds to `s = -1`
- `x = 1` corresponds to `s = +1`

The library provides bidirectional reductions between them.
