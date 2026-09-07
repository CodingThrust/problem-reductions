# Discover and execute paths

Search uses exact variants. This example discovers a witness-capable route from `Factoring` to `SpinGlass`.

```rust,ignore
use problemreductions::prelude::*;
use problemreductions::rules::{MinimizeSteps, ReductionGraph};
use problemreductions::topology::SimpleGraph;
use problemreductions::types::ProblemSize;
{{#include ../../examples/chained_reduction_factoring_to_spinglass.rs:step1}}
```

## Execute the route

```rust,ignore
let factoring = Factoring::new(2, 2, 6);
let reduction = graph
    .reduce_along_path(&rpath, &factoring)
    .unwrap();
let target: &SpinGlass<SimpleGraph, f64> = reduction.target_problem();
// Solve `target` with a compatible solver, then recover source bits:
// let source_config = reduction.extract_solution(&target_config);
```

`extract_solution` walks the intermediate mappings in reverse. A discovered path establishes a registered transformation, not an efficient target solve.

The [runnable factoring example](https://github.com/CodingThrust/problem-reductions/blob/main/examples/chained_reduction_factoring_to_spinglass.rs) discovers the SpinGlass route, then separately solves factoring through a direct integer ILP reduction. Run it with:

```bash
cargo run --example chained_reduction_factoring_to_spinglass
```

It checks that the recovered factors multiply to 6 and prints composed overheads.

Next: [path costs and overhead](design-paths.md).
