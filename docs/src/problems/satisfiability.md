# Satisfiability Problems

Satisfiability problems involve finding assignments to boolean variables that satisfy given constraints.

## SAT (Boolean Satisfiability)

The Boolean Satisfiability Problem (SAT) asks whether there exists an assignment of truth values to variables that makes a given boolean formula evaluate to TRUE.

**Definition**: Given a CNF (Conjunctive Normal Form) formula with n variables and m clauses, find an assignment that satisfies all clauses.

A CNF formula is a conjunction (AND) of clauses, where each clause is a disjunction (OR) of literals. A literal is either a variable (x) or its negation (NOT x).

```rust
use problemreductions::prelude::*;

// Formula: (x1 OR x2) AND (NOT x1 OR x3)
let problem = Satisfiability::<i32>::new(
    3,  // 3 variables
    vec![
        CNFClause::new(vec![1, 2]),     // x1 OR x2
        CNFClause::new(vec![-1, 3]),    // NOT x1 OR x3
    ],
);

let solver = BruteForce::new();
let solutions = solver.find_best(&problem);
```

**Complexity**: SAT is NP-complete (Cook-Levin theorem, 1971).

### KSatisfiability (k-SAT)

A restricted version where every clause has exactly k literals.

```rust
use problemreductions::prelude::*;

// 3-SAT formula: each clause has exactly 3 literals
let problem = KSatisfiability::<3, i32>::new(
    4,  // 4 variables
    vec![
        KSATClause::new([1, 2, 3]),      // x1 OR x2 OR x3
        KSATClause::new([-1, -2, 4]),    // NOT x1 OR NOT x2 OR x4
    ],
);
```

**Note**: 3-SAT remains NP-complete, while 2-SAT is in P.

## CircuitSAT

CircuitSAT extends SAT to boolean circuits. Instead of a CNF formula, the input is a circuit of logic gates (AND, OR, NOT, XOR).

**Definition**: Given a boolean circuit, find an assignment to input variables such that the circuit outputs TRUE.

```rust
use problemreductions::prelude::*;

// Circuit computing: output = (x AND y) XOR z
let circuit = Circuit::new(vec![
    Assignment::new(
        vec!["temp".to_string()],
        BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
    ),
    Assignment::new(
        vec!["output".to_string()],
        BooleanExpr::xor(vec![BooleanExpr::var("temp"), BooleanExpr::var("z")]),
    ),
]);
let problem = CircuitSAT::<i32>::new(circuit);

let solver = BruteForce::new();
let solutions = solver.find_best(&problem);
```

### Supported Gate Types

- `BooleanExpr::and(inputs)` - AND gate
- `BooleanExpr::or(inputs)` - OR gate
- `BooleanExpr::not(input)` - NOT gate
- `BooleanExpr::xor(inputs)` - XOR gate
- `BooleanExpr::var(name)` - Input variable
- `BooleanExpr::constant(value)` - Constant TRUE/FALSE

**Complexity**: CircuitSAT is NP-complete.

### Reduction to SpinGlass

CircuitSAT can be reduced to SpinGlass (Ising model) using gate gadgets. Each logic gate is mapped to a set of spin interactions that encode the gate's truth table as energy penalties.

```rust
use problemreductions::prelude::*;

let circuit = Circuit::new(vec![
    Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
    ),
]);
let problem = CircuitSAT::<i32>::new(circuit);

// Reduce to SpinGlass
let reduction = ReduceTo::<SpinGlass<i32>>::reduce_to(&problem);
let sg = reduction.target_problem();
```

## Factoring

Integer factorization expressed as a decision problem. Given a composite number N, find non-trivial factors p and q such that p * q = N.

**Definition**: Given N and bit sizes m and n, find m-bit integer p and n-bit integer q such that p * q = N.

```rust
use problemreductions::prelude::*;

// Factor 15 using 4-bit factors
// Arguments: (m bits for p, n bits for q, target N)
let problem = Factoring::new(4, 4, 15);

// Read a candidate solution
let solution = vec![1, 1, 0, 0, 1, 0, 1, 0]; // p=3, q=5 in little-endian
let (p, q) = problem.read_factors(&solution);
assert_eq!(p * q, 15);
```

### How it Works

The `Factoring` problem stores:
- `m`: Number of bits for the first factor
- `n`: Number of bits for the second factor
- `target`: The number to factor (N)

Solutions are bit vectors where:
- First m bits encode p (little-endian)
- Next n bits encode q (little-endian)

### Reduction to CircuitSAT

Factoring reduces to CircuitSAT by constructing a multiplier circuit:

```rust
use problemreductions::prelude::*;

let factoring = Factoring::new(4, 4, 15);

// Reduce to CircuitSAT (multiplier circuit)
let reduction = ReduceTo::<CircuitSAT<i32>>::reduce_to(&factoring);
let circuit_sat = reduction.target_problem();

// The circuit computes p * q and constrains output = 15
// Solving gives the factors
```

The reduction builds an array multiplier circuit with:
- Input variables for p (m bits) and q (n bits)
- Multiplier cells computing partial products
- Output constraints fixing the product to equal N

**Complexity**: Factoring is believed to be hard classically but in BQP (solvable by quantum computers via Shor's algorithm).

## Problem Relationships

```
Factoring --> CircuitSAT --> SpinGlass
                  |
                  v
            Satisfiability --> IndependentSet
                  |                  |
                  v                  v
              Coloring          SetPacking
                  |
                  v
           DominatingSet
```

All these problems are NP-complete (except Factoring, whose complexity is unknown but believed to be intermediate).
