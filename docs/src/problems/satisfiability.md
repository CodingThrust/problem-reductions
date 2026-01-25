# Satisfiability Problems

## SAT (Boolean Satisfiability)

Find assignments satisfying a CNF formula.

```rust
use problemreductions::prelude::*;

let problem = Satisfiability::<i32>::new(
    3,
    vec![
        CNFClause::new(vec![1, 2]),     // x1 OR x2
        CNFClause::new(vec![-1, 3]),    // NOT x1 OR x3
    ],
);

let solver = BruteForce::new();
let solutions = solver.find_best(&problem);
```

## CircuitSAT

Satisfy a boolean circuit with gates.

```rust
use problemreductions::prelude::*;

let circuit = Circuit::new(vec![
    Assignment::new(
        vec!["output".to_string()],
        BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
    ),
]);
let problem = CircuitSAT::<i32>::new(circuit);
```

## Factoring

Integer factorization as a decision problem.

```rust
use problemreductions::prelude::*;

// Factor 15 with 2-bit factors
let problem = Factoring::new(15, 2, 2);
```
