# Plan: TravelingSalesman to QUBO Reduction (#167)

## Overview
Add reduction from TravelingSalesman to QUBO using position-based encoding from Lucas (2014).

## Key Design Decisions

**Variable encoding:** n² binary variables x_{v,p} where x_{v,p}=1 means city v at tour position p. QUBO variable index = v*n + p.

**Penalty coefficient:** A = 1 + sum of all edge weights (ensures constraint violations dominate).

**Non-complete graphs:** The issue formulation sums H_C only over edges in E. For non-complete graphs, we must also add penalty A for non-edge consecutive pairs, otherwise the QUBO ground state may place non-adjacent cities consecutively (yielding invalid tours). Implementation: iterate over ALL vertex pairs (u,v); use w_{uv} for edges, penalty A for non-edges.

**Solution extraction:** Decode position encoding (find v where x_{v,p}=1 for each p) → build tour order → convert to edge-based config (TSP uses one binary variable per edge).

**Weight type:** QUBO<f64> (consistent with all other QUBO reductions).

**Impl type params:** `TravelingSalesman<SimpleGraph, i32>` (matches the only declared variant).

## Tasks (parallelizable: 1-3 independent, then 4-5)

### Task 1: Reduction rule (`src/rules/travelingsalesman_qubo.rs`)

**File:** `src/rules/travelingsalesman_qubo.rs`

Create the reduction following the KColoring→QUBO pattern:

```rust
struct ReductionTravelingSalesmanToQUBO {
    target: QUBO<f64>,
    num_vertices: usize,
    edge_set: HashSet<(usize, usize)>,  // for extract_solution
    edge_index: HashMap<(usize, usize), usize>,  // (u,v) → edge index in TSP
}
```

**reduce_to():**
1. Get n = num_vertices, edges, edge_weights
2. Compute A = 1.0 + sum of all |edge weights| (as f64)
3. Build n² × n² matrix:
   - H_A (row constraints): diagonal Q[v*n+p][v*n+p] -= A; off-diag Q[v*n+p][v*n+p'] += 2A for p<p'
   - H_B (column constraints): diagonal Q[v*n+p][v*n+p] -= A; off-diag Q[v*n+p][v'*n+p] += 2A for v<v'
   - H_C (distance): for all pairs (u,v) with u<v, for each p: add cost to Q[u*n+p][v*n+(p+1)%n] and Q[v*n+p][u*n+(p+1)%n]. Cost = w_{uv} for edges, A for non-edges.
4. Ensure upper-triangular: when adding to Q[i][j], always use min(i,j), max(i,j)

**extract_solution():**
1. Decode position: for each p in 0..n, find v where target_solution[v*n+p] == 1
2. Build tour_order: vertex at each position
3. Convert to edge config: for each consecutive pair (tour[p], tour[(p+1)%n]), find edge index and set config[edge_idx] = 1

**Registration:**
```rust
#[reduction(overhead = { num_vars = "num_vertices^2" })]
impl ReduceTo<QUBO<f64>> for TravelingSalesman<SimpleGraph, i32> { ... }
```

**Register in `src/rules/mod.rs`:** Add `mod travelingsalesman_qubo;`

### Task 2: Unit tests (`src/unit_tests/rules/travelingsalesman_qubo.rs`)

Reference: `src/unit_tests/rules/coloring_qubo.rs`

**Tests:**
1. `test_travelingsalesman_to_qubo_closed_loop` — K3 complete graph with weights [1, 2, 3]. Verify all QUBO optimal solutions extract to valid Hamiltonian cycles with minimum cost 6.
2. `test_travelingsalesman_to_qubo_k4` — K4 complete graph, verify solution extraction and optimal tour.
3. `test_travelingsalesman_to_qubo_sizes` — Check QUBO has n² variables for n-vertex graph.
4. `test_travelingsalesman_to_qubo_non_complete` — Triangle + pendant (non-Hamiltonian). Verify QUBO penalizes non-edge transitions.

Link test file from rule file: `#[cfg(test)] #[path = "..."] mod tests;`

### Task 3: Example (`examples/reduction_travelingsalesman_to_qubo.rs`)

Reference: `examples/reduction_kcoloring_to_qubo.rs`

Instance: K3 with weights w01=1, w02=2, w12=3 (from issue).
- Create TravelingSalesman with SimpleGraph K3 and i32 weights
- Reduce to QUBO, solve with BruteForce
- Extract solutions, verify closed-loop
- Export JSON with write_example()
- Register in tests/suites/examples.rs

### Task 4: Register example in tests

Add `include!` entry in `tests/suites/examples.rs` for the new example.

### Task 5: Regenerate data files

Run `make examples` and `make export-schemas` to update JSON data files used by the paper and tests.
