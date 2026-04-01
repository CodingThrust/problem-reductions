---
name: add-reduction
description: Add a new reduction rule using verified artifacts from /verify-reduction — reads Python reduce() as pseudocode, test vectors JSON for Rust tests, overhead from JSON
---

# Add Reduction (from Verified Artifacts)

Step-by-step guide for adding a new reduction rule (A → B) when `/verify-reduction` has already produced verified artifacts (Typst proof, Python scripts, test vectors JSON). This skill consumes those artifacts directly instead of re-deriving from the issue.

**When to use:** After `/verify-reduction` has produced a PR with verified artifacts for a reduction rule issue. Use `/add-rule` instead when no verification artifacts exist.

## Step 0: Locate Verified Artifacts

Check for existing verification artifacts:

```bash
ls docs/paper/verify-reductions/verify_<source>_<target>.py
ls docs/paper/verify-reductions/test_vectors_<source>_<target>.json
ls docs/paper/verify-reductions/<source>_<target>.typ
```

If any are missing, run `/verify-reduction` first.

### Read the artifacts

1. **Python `reduce()` function** — this is the verified spec for the Rust `reduce_to()` implementation. Read it carefully; translate the algorithm, not the syntax.
2. **Test vectors JSON** — contains YES/NO instances with exact input/output values, overhead expressions, and verified claims.
3. **Typst proof** — the Construction section describes the algorithm in mathematical notation. Use for doc comments.

```bash
# Load test vectors
TEST_VECTORS=$(cat docs/paper/verify-reductions/test_vectors_<source>_<target>.json)
```

Extract from test vectors JSON:
- `overhead` → use directly in `#[reduction(overhead = { ... })]`
- `yes_instance.input` / `yes_instance.output` → first closed-loop test case
- `no_instance.input` / `no_instance.output` → infeasible test case
- `claims` → verify each is preserved in the Rust implementation

## Reference Implementations

Same as `/add-rule`:
- **Reduction rule:** `src/rules/minimumvertexcover_maximumindependentset.rs`
- **Reduction tests:** `src/unit_tests/rules/minimumvertexcover_maximumindependentset.rs`
- **Paper entry:** search `docs/paper/reductions.typ` for `MinimumVertexCover` `MaximumIndependentSet`
- **Traits:** `src/rules/traits.rs` (`ReduceTo<T>`, `ReduceToAggregate<T>`, `ReductionResult`, `AggregateReductionResult`)

## Step 1: Implement the reduction

Create `src/rules/<source>_<target>.rs`.

**Translation guide:** Map the Python `reduce()` function to Rust:

| Python | Rust |
|--------|------|
| `reduce(n, clauses)` → `(universe_size, subsets)` | `fn reduce_to(&self) -> Self::Result` |
| `extract_assignment(n, config)` | `fn extract_solution(&self, target_sol: &[usize]) -> Vec<usize>` |
| `literal_to_element(lit, n)` | Private helper method |
| Python list of ints | `Vec<usize>`, `Vec<CNFClause>`, etc. (match problem type) |

**Overhead from test vectors JSON:** The `overhead` field maps directly to the `#[reduction]` macro:

```rust
#[reduction(overhead = {
    // Copy expressions verbatim from test_vectors JSON "overhead" field
    field_name = "expression",
})]
```

The rest of the implementation structure follows `/add-rule` Step 1 exactly: ReductionResult struct, trait impl, ReduceTo impl.

## Step 2: Register in mod.rs

Same as `/add-rule` Step 2. Add `mod <source>_<target>;` to `src/rules/mod.rs`.

## Step 3: Write unit tests from test vectors

Create `src/unit_tests/rules/<source>_<target>.rs`.

**Generate tests directly from test vectors JSON:**

The YES instance becomes the primary closed-loop test:

```rust
#[test]
fn test_<source>_to_<target>_closed_loop() {
    // Construct source from test_vectors.yes_instance.input
    let source = <SourceType>::try_new(/* fields from JSON */).unwrap();

    // Reduce
    let reduction = ReduceTo::<TargetType>::reduce_to(&source);

    // Verify target matches test_vectors.yes_instance.output
    let target = reduction.target_problem();
    assert_eq!(target.<field>(), /* value from JSON output */);

    // Solve and extract
    let solver = BruteForce;
    for witness in solver.find_all_witnesses(target).unwrap() {
        let extracted = reduction.extract_solution(&witness);
        // Verify extracted solution is valid for source
        let val = source.evaluate(&extracted);
        assert!(val.0); // Or check objective value
    }
}
```

The NO instance becomes the infeasible test:

```rust
#[test]
fn test_<source>_to_<target>_infeasible() {
    // Construct source from test_vectors.no_instance.input
    let source = <SourceType>::try_new(/* fields from JSON */).unwrap();

    // Reduce
    let reduction = ReduceTo::<TargetType>::reduce_to(&source);

    // Verify target is also infeasible
    let solver = BruteForce;
    let witnesses = solver.find_all_witnesses(reduction.target_problem());
    assert!(witnesses.is_none() || witnesses.unwrap().is_empty());
}
```

Add additional structural tests as needed (target size, edge count, etc.) guided by the `claims` field in the test vectors JSON.

## Step 4: Add canonical example to example_db

Same as `/add-rule` Step 4. The YES instance from the test vectors JSON is a good candidate for the canonical example.

## Step 5: Document in paper

The Typst proof already exists from `/verify-reduction`. Integrate it into `docs/paper/reductions.typ` using the `reduction-rule` template. The proof text, worked examples, and overhead table are already written — adapt them to the paper's macros (`reduction-rule`, `problem-def`, etc.).

Follow `/add-rule` Step 5 for the exact format. The heavy writing is already done; this step is reformatting.

## Step 6: Regenerate exports and verify

Same as `/add-rule` Step 6:

```bash
cargo run --example export_graph
cargo run --example export_schemas
make regenerate-fixtures
make test clippy
```

## Solver Rules

Same as `/add-rule`. If the target problem needs ILP, implement alongside.

## CLI Impact

Same as `/add-rule`. No CLI changes needed for witness-preserving reductions.

## File Naming

Same as `/add-rule`:
- Rule file: `src/rules/<sourcelower>_<targetlower>.rs`
- Test file: `src/unit_tests/rules/<sourcelower>_<targetlower>.rs`
- Canonical example: builder function in `src/example_db/rule_builders.rs`

## Common Mistakes

All mistakes from `/add-rule` apply, plus:

| Mistake | Fix |
|---------|-----|
| Re-deriving algorithm from issue instead of reading Python `reduce()` | The Python function is the verified spec — translate it, don't reinvent |
| Ignoring test vectors JSON | Use the YES/NO instances for Rust tests directly |
| Overhead expressions don't match test vectors JSON | Copy verbatim from the `overhead` field |
| Skipping the infeasible (NO) test case | The NO instance is in the test vectors — always include it |
| Not integrating the existing Typst proof into the paper | The proof is already written; reformat, don't rewrite |
