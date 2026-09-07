# First Rust program

Add the library to a Rust project:

```bash
cargo add problemreductions
```

## Solve a small set-packing instance

```rust
use problemreductions::prelude::*;

fn main() {
    let problem = MaximumSetPacking::<i32>::new(vec![
        vec![0, 1], vec![1, 2], vec![2, 3], vec![4, 5],
    ]);
    let solver = BruteForce::new();
    let solution = solver.find_witness(&problem).unwrap();
    println!("{:?}: {}", solution, problem.evaluate(&solution));
}
```

The optimal packing selects sets 0, 2, and 3 and evaluates to `Max(3)`. The witness vector is `[1,0,1,1]`.

`Problem::evaluate` checks a configuration; `BruteForce` enumerates the configuration space to find an optimum. Keep exhaustive examples small.

Next: [apply an ILP reduction](rust-reduction.md) or [read solver contracts](rust-solvers.md).
