# Plan: Add SteinerTreeInGraphs Model

**Issue:** #255 — [Model] SteinerTreeInGraphs
**Skill:** add-model

## Information Checklist

| # | Item | Value |
|---|------|-------|
| 1 | Problem name | `SteinerTreeInGraphs` |
| 2 | Mathematical definition | Given G=(V,E) with edge weights w(e) >= 0 and terminals R ⊆ V, find a subtree T of G spanning all terminals R that minimizes Σ w(e) for e ∈ T |
| 3 | Problem type | Optimization (Minimize) |
| 4 | Type parameters | `G: Graph`, `W: WeightElement` |
| 5 | Struct fields | `graph: G`, `terminals: Vec<usize>`, `edge_weights: Vec<W>` |
| 6 | Configuration space | `vec![2; num_edges]` — binary edge selection |
| 7 | Feasibility check | Selected edges form a connected subtree spanning all terminals in R |
| 8 | Objective function | Sum of weights of selected edges |
| 9 | Best known exact algorithm | Dreyfus-Wagner DP: O(3^k * n + 2^k * n^2 + n^3), where k = |R| (terminals), n = |V| |
| 10 | Solving strategy | BruteForce (enumerate edge subsets) |
| 11 | Category | `graph` |

## Step 1: Create model file

Create `src/models/graph/steiner_tree_in_graphs.rs`:

- `inventory::submit!` for ProblemSchemaEntry with fields: graph, terminals, edge_weights
- Struct `SteinerTreeInGraphs<G, W>` with fields: graph, terminals, edge_weights
- Constructor `new(graph, terminals, edge_weights)` with assertions
- Accessor methods: `graph()`, `terminals()`, `weights()`, `edges()`
- Weight management: `set_weights()`, `is_weighted()`
- Size getters: `num_vertices()`, `num_edges()`, `num_terminals()`
- `is_valid_solution()` helper
- `Problem` impl: NAME="SteinerTreeInGraphs", Metric=SolutionSize<W::Sum>, dims=vec![2; num_edges]
- `evaluate()`: check if edges form connected subgraph spanning all terminals, then sum weights
- `OptimizationProblem` impl: direction = Minimize
- Feasibility helper function: `is_steiner_tree(graph, terminals, selected_edges) -> bool`
  - Check: selected edges form a tree (connected, no cycles or just connected acyclic subgraph spanning terminals)
  - Actually: any connected subgraph spanning all terminals is valid (not necessarily a tree, but optimal will always be a tree). For evaluate, check connectivity of terminals through selected edges.
- `declare_variants!` with Dreyfus-Wagner complexity: `"3^num_terminals * num_vertices"`
- `#[cfg(test)] #[path]` link to unit test file

## Step 2: Register the model

1. `src/models/graph/mod.rs` — add `pub(crate) mod steiner_tree_in_graphs;` and `pub use steiner_tree_in_graphs::SteinerTreeInGraphs;`
2. `src/models/mod.rs` — add `SteinerTreeInGraphs` to the graph re-export line

## Step 3: Register in CLI

1. `problemreductions-cli/src/dispatch.rs`:
   - Add `"SteinerTreeInGraphs"` arm in `load_problem()` → `deser_opt::<SteinerTreeInGraphs<SimpleGraph, i32>>(data)`
   - Add `"SteinerTreeInGraphs"` arm in `serialize_any_problem()` → `try_ser::<SteinerTreeInGraphs<SimpleGraph, i32>>(any)`
2. `problemreductions-cli/src/problem_name.rs`:
   - Add `"steinertreeingraphs" => "SteinerTreeInGraphs".to_string()` in `resolve_alias()`
3. `problemreductions-cli/src/commands/create.rs`:
   - Add `"SteinerTreeInGraphs"` arm: parse --graph, --edge-weights, --terminals
   - Add to random generation support
4. `problemreductions-cli/src/cli.rs`:
   - Add `--terminals` flag to CreateArgs: `pub terminals: Option<String>`
   - Update `all_data_flags_empty()` to include `args.terminals.is_none()`
   - Add to "Flags by problem type" table: `SteinerTreeInGraphs --graph, --edge-weights, --terminals`

## Step 4: Write unit tests

Create `src/unit_tests/models/graph/steiner_tree_in_graphs.rs`:
- `test_steiner_tree_creation` — construct instance, verify dimensions
- `test_steiner_tree_evaluation` — verify evaluate() on valid and invalid configs
- `test_steiner_tree_direction` — verify Minimize
- `test_steiner_tree_solver` — brute-force solver finds correct optimal
- `test_steiner_tree_is_valid_solution` — test validity checker
- `test_steiner_tree_size_getters` — test num_vertices, num_edges, num_terminals

## Step 5: Document in paper

Add problem-def entry in `docs/paper/reductions.typ`:
- Add `"SteinerTreeInGraphs": [Steiner Tree in Graphs]` to display-name dict
- Add `#problem-def("SteinerTreeInGraphs")[...][...]` section with formal definition, background, and example

## Step 6: Verify

Run `make test clippy` to ensure everything passes.
