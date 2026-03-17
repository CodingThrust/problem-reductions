# Plan: MinimumMultiwayCut to QUBO Reduction (#186)

## Summary

Implement the reduction from MinimumMultiwayCut to QUBO based on Heidari, Dinneen & Delmas (2022). The reduction uses kn binary variables where k = |terminals| and n = |vertices|, encoding vertex-to-terminal assignments via a penalty Hamiltonian H_A (enforces valid partition + terminal pinning) and a cut-cost Hamiltonian H_B.

## Batch 1: Implementation (Steps 1-4)

All tasks in this batch are independent and can be parallelized.

### Task 1.1: Implement reduction rule

**File:** `src/rules/minimummultiwaycut_qubo.rs`

**Reduction algorithm:**
- Variable mapping: kn binary variables x_{u,t} indexed as `u * k + pos(t)` where pos(t) is the position of terminal t in the terminals list
- alpha = penalty coefficient > sum of all edge weights (use sum + 1.0)

**Q-matrix construction (expanding H = H_A + H_B):**

H_A = alpha * (sum_u (1 - sum_t x_{u,t})^2 + sum_t sum_{t'!=t} x_{t,t'})

Expanding (1 - sum_t x_{u,t})^2 with x^2=x for binary:
= 1 - sum_t x_{u,t} + 2 * sum_{s<t} x_{u,s} * x_{u,t}

So H_A contributes:
1. **Diagonal** (one-hot penalty): Q[u*k+s, u*k+s] -= alpha, for all u, s
2. **Off-diagonal** (one-hot cross): Q[u*k+s, u*k+t] += 2*alpha, for all u, s < t
3. **Diagonal** (terminal pinning): Q[t_idx*k+s, t_idx*k+s] += alpha, for each terminal t_idx and each terminal position s != pos(t_idx)

H_B = sum_{(u,v) in E} sum_s sum_{t!=s} C({u,v}) * x_{u,s} * x_{v,t}

H_B contributes:
4. **Off-diagonal** (cut cost): Q[min(u*k+s, v*k+t), max(u*k+s, v*k+t)] += C({u,v}), for each edge (u,v) and distinct terminal positions s != t

**ReductionResult struct:** stores target QUBO, source graph reference data (num_vertices, num_edges, terminal list, edges list) for solution extraction.

**Solution extraction:** From QUBO solution (vertex assignments), derive edge cut: for each edge (u,v), if assigned to different terminals, mark as cut (1), else keep (0).

**Overhead:** `num_vars = "num_terminals * num_vertices"`

**Reference patterns:** `src/rules/coloring_qubo.rs` (one-hot encoding + penalty QUBO)

### Task 1.2: Register in mod.rs

**File:** `src/rules/mod.rs`
- Add `pub(crate) mod minimummultiwaycut_qubo;` (alphabetical order, after `minimumvertexcover_minimumsetcovering`)
- Add `specs.extend(minimummultiwaycut_qubo::canonical_rule_example_specs());` in `canonical_rule_example_specs()`

### Task 1.3: Write unit tests

**File:** `src/unit_tests/rules/minimummultiwaycut_qubo.rs`

Tests:
1. `test_minimummultiwaycut_to_qubo_closed_loop` — Use the issue example (5 vertices, 3 terminals {0,2,4}, 6 weighted edges). Reduce, solve QUBO with BruteForce, extract solutions, verify each extracted solution has source cost = 8 (optimal).
2. `test_minimummultiwaycut_to_qubo_small` — Triangle graph, 2 terminals. Verify correct QUBO size and valid extraction.
3. `test_minimummultiwaycut_to_qubo_sizes` — Verify QUBO has k*n variables.
4. `test_minimummultiwaycut_to_qubo_terminal_pinning` — Verify terminals are correctly pinned (each terminal assigned to its own component in all optimal solutions).

### Task 1.4: Add canonical example to example_db

**File:** `src/rules/minimummultiwaycut_qubo.rs` (add `canonical_rule_example_specs` function)

Use the issue example: 5 vertices, terminals {0,2,4}, edges with weights [2,3,1,2,4,5]. Use `direct_best_example` with `|_, _| true` keep function (since MinimumMultiwayCut is an optimization problem).

## Batch 2: Paper Documentation (Step 5)

Depends on Batch 1 completion (needs exports).

### Task 2.1: Write paper entry + regenerate exports

**File:** `docs/paper/reductions.typ`

1. Run `cargo run --example export_graph` and `cargo run --example export_schemas`
2. Add `reduction-rule("MinimumMultiwayCut", "QUBO", ...)` entry with:
   - Rule statement: O(kn) reduction, kn binary variables encoding vertex-to-terminal assignments
   - Proof: Construction (H_A + H_B), Correctness (bidirectional), Solution extraction
   - Worked example from JSON fixture data
3. Run `make paper` to verify compilation

## Verification

After all batches:
```bash
make test clippy
cargo run --example export_graph
cargo run --example export_schemas
make paper
```
