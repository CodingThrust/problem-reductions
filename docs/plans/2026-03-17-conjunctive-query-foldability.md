# Plan: Add ConjunctiveQueryFoldability Model

**Issue:** #448
**Skill:** add-model
**Category:** `misc` (unique input structure: domain, relations, conjunctive queries)

## Problem Summary

Conjunctive Query Foldability (Garey & Johnson A4 SR30). Given two conjunctive queries Q1 and Q2 over a finite domain D with relations R, distinguished variables X, and undistinguished variables Y, determine if there exists a substitution σ: Y → X ∪ Y ∪ D such that applying σ to Q1 produces Q2.

- **Type:** Satisfaction (`Metric = bool`)
- **NP-complete:** Chandra & Merlin 1977, via reduction from Graph 3-Colorability
- **Best known:** Brute-force enumeration of all substitutions
- **No type parameters** (no G, W)

## Batch 1: Implementation (Steps 1–5.5)

### Task 1.1: Create model file `src/models/misc/conjunctive_query_foldability.rs`

**Term enum:**
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Term {
    Constant(usize),
    Distinguished(usize),
    Undistinguished(usize),
}
```

**Struct:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConjunctiveQueryFoldability {
    domain_size: usize,
    num_distinguished: usize,
    num_undistinguished: usize,
    relation_arities: Vec<usize>,
    query1_conjuncts: Vec<(usize, Vec<Term>)>,
    query2_conjuncts: Vec<(usize, Vec<Term>)>,
}
```

**ProblemSchemaEntry:** Register with `inventory::submit!`. Fields: `domain_size`, `num_distinguished`, `num_undistinguished`, `relation_arities`, `query1_conjuncts`, `query2_conjuncts`.

**Constructor `new()`:** Validate that:
- All relation indices in conjuncts are < relation_arities.len()
- Each atom's argument count matches the relation's arity
- All term indices are in range (Constant < domain_size, Distinguished < num_distinguished, Undistinguished < num_undistinguished)

**Size getters** (for overhead expressions + declare_variants):
- `domain_size()` → `self.domain_size`
- `num_distinguished()` → `self.num_distinguished`
- `num_undistinguished()` → `self.num_undistinguished`
- `num_conjuncts_q1()` → `self.query1_conjuncts.len()`
- `num_conjuncts_q2()` → `self.query2_conjuncts.len()`
- `num_relations()` → `self.relation_arities.len()`

**Problem trait:**
- `NAME = "ConjunctiveQueryFoldability"`
- `type Metric = bool`
- `dims()` → `vec![num_distinguished + num_undistinguished + domain_size; num_undistinguished]`
  - Each undistinguished variable can map to any of: distinguished vars, undistinguished vars, or domain constants
- `evaluate(config)`:
  1. Decode σ from config: for each undistinguished var i, config[i] maps to a target term (0..domain_size → Constant, then Distinguished, then Undistinguished)
  2. Apply σ to every atom in Q1: replace each Undistinguished(i) with σ(i)
  3. Collect substituted atoms as a HashSet
  4. Return true iff the set equals Q2's atoms as a HashSet
- `variant()` → `crate::variant_params![]` (no type parameters)

**SatisfactionProblem:** Implement marker trait.

**declare_variants!:**
```rust
crate::declare_variants! {
    default sat ConjunctiveQueryFoldability => "(num_distinguished + num_undistinguished + domain_size)^num_undistinguished * num_conjuncts_q1",
}
```

**Test link:** `#[cfg(test)] #[path = "../../unit_tests/models/misc/conjunctive_query_foldability.rs"] mod tests;`

### Task 1.2: Register in module tree

- `src/models/misc/mod.rs`: Add `mod conjunctive_query_foldability;` and `pub use conjunctive_query_foldability::ConjunctiveQueryFoldability;`. Also add `pub use conjunctive_query_foldability::Term;` for the Term enum. Update module doc comment. Add to `canonical_model_example_specs()`.
- `src/models/mod.rs`: Add to the re-export line for `misc`.

### Task 1.3: CLI alias registration

In `problemreductions-cli/src/problem_name.rs`:
- Add `"conjunctivequeryfoldability" => "ConjunctiveQueryFoldability"` to `resolve_alias()`
- Do NOT add a short alias — "CQF" is not well-established in the literature

### Task 1.4: CLI create support

In `problemreductions-cli/src/commands/create.rs`:
- Add a match arm for `"ConjunctiveQueryFoldability"` in the main match block
- Parse required flags: `--domain-size`, `--num-distinguished`, `--num-undistinguished`, `--relation-arities` (comma-separated), `--query1` (JSON array of atoms), `--query2` (JSON array of atoms)
- Add any needed flags to `CreateArgs` in `cli.rs`
- Update help text table

### Task 1.5: Add canonical model example

In `src/models/misc/conjunctive_query_foldability.rs`, add:
```rust
#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> { ... }
```

Use the YES instance from the issue:
- Domain size 0, 1 distinguished var, 3 undistinguished vars
- Single binary relation R (arity 2)
- Q1: R(x,u) ∧ R(u,v) ∧ R(v,x) ∧ R(u,u) (triangle + self-loop)
- Q2: R(x,a) ∧ R(a,a) ∧ R(a,x) (lollipop)
- Solution: σ(U0→U2, U1→U2) → config = [2, 2, _] where index 2 maps to Undistinguished(2)

### Task 1.6: Write unit tests

Create `src/unit_tests/models/misc/conjunctive_query_foldability.rs`:

1. `test_conjunctive_query_foldability_creation` — construct instance, verify dims
2. `test_conjunctive_query_foldability_yes_instance` — YES example: evaluate the known satisfying config, assert true
3. `test_conjunctive_query_foldability_no_instance` — NO example (triangle → 2-cycle): brute-force finds no satisfying config
4. `test_conjunctive_query_foldability_solver` — BruteForce::find_satisfying on YES instance returns Some
5. `test_conjunctive_query_foldability_serialization` — round-trip serde test
6. `test_conjunctive_query_foldability_paper_example` — same instance as paper, verify expected outcome and solution count

Update `src/unit_tests/models/misc/mod.rs` to include the test module.

### Task 1.7: Add trait_consistency entry

In `src/unit_tests/trait_consistency.rs`:
- Add `check_problem_trait(...)` call with a small instance (the YES example)

### Task 1.8: Build and test

```bash
make fmt
make clippy
make test
```

## Batch 2: Paper Entry (Step 6)

### Task 2.1: Write paper entry in `docs/paper/reductions.typ`

**Display name:** Add `"ConjunctiveQueryFoldability": [Conjunctive Query Foldability]` to the `display-name` dictionary.

**problem-def:** Write `#problem-def("ConjunctiveQueryFoldability")[def][body]`:
- **Definition:** Given a finite domain D, relations R, distinguished variables X, undistinguished variables Y, and two conjunctive queries Q1, Q2 — determine whether there exists a substitution σ: Y → X ∪ Y ∪ D such that applying σ to every undistinguished variable in Q1 produces Q2.
- **Background:** Classical NP-complete problem in database theory (Chandra & Merlin, 1977). Fundamental to query optimization — if Q1 folds into Q2, then Q1 is redundant and can be eliminated. Equivalent to testing the existence of a homomorphism between the canonical databases of the two queries.
- **Algorithm:** Brute-force: enumerate all |X ∪ Y ∪ D|^|Y| substitutions.
- **Example:** CeTZ diagram showing the YES instance (triangle + self-loop → lollipop). Show Q1's atoms, Q2's atoms, the substitution σ, and the result.

### Task 2.2: Build paper and verify

```bash
make paper
```

Verify the paper compiles and the example renders correctly.

## Dependencies

- Batch 2 depends on Batch 1 (needs exports from compiled model)
- All tasks within Batch 1 are independent except: Task 1.6 depends on Task 1.1, Task 1.5 depends on Task 1.1
- Task 1.8 depends on all other Batch 1 tasks
