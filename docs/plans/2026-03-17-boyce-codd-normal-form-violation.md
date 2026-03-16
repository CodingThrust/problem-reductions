# Plan: Add BoyceCoddNormalFormViolation Model

**Issue:** #447
**Type:** [Model] — Satisfaction problem
**Category:** `misc/` (unique input structure: functional dependencies on attributes)
**Reference:** Beeri & Bernstein, 1979; Garey & Johnson A4 SR29

## Problem Summary

Given a set A of attributes, a collection F of functional dependencies on A, and a subset A' ⊆ A, determine whether A' violates Boyce-Codd Normal Form (BCNF). A violation exists when there is a subset X ⊆ A' and two attributes y, z ∈ A' \ X such that y ∈ X⁺ (the closure of X under F) but z ∉ X⁺ — meaning X determines y but not z, so X is not a superkey.

This is a satisfaction problem: the binary variables encode which attributes are in X, and `evaluate()` returns true iff that X witnesses a BCNF violation.

## Batch 1: Implementation (Steps 1–5.5)

### Step 1: Create model file `src/models/misc/boyce_codd_normal_form_violation.rs`

**Inventory registration:**
```rust
inventory::submit! {
    ProblemSchemaEntry {
        name: "BoyceCoddNormalFormViolation",
        display_name: "Boyce-Codd Normal Form Violation",
        aliases: &["BCNFViolation", "BCNF"],
        dimensions: &[],
        module_path: "models::misc::boyce_codd_normal_form_violation",
        description: "Test whether a subset of attributes violates Boyce-Codd normal form",
        fields: &["num_attributes", "functional_deps", "target_subset"],
    }
}
```

**Struct:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoyceCoddNormalFormViolation {
    num_attributes: usize,
    functional_deps: Vec<(Vec<usize>, Vec<usize>)>,
    target_subset: Vec<usize>,
}
```

**Constructor `new(num_attributes, functional_deps, target_subset)`:**
- Validate: all attribute indices in functional_deps and target_subset are < num_attributes
- Validate: target_subset is non-empty, sorted, deduplicated
- Validate: each FD has non-empty LHS
- Sort and dedup each FD's LHS and RHS
- Sort and dedup target_subset

**Getter methods:**
- `num_attributes(&self) -> usize` — returns `self.num_attributes`
- `num_functional_deps(&self) -> usize` — returns `self.functional_deps.len()`
- `num_target_attributes(&self) -> usize` — returns `self.target_subset.len()`
- `functional_deps(&self) -> &[(Vec<usize>, Vec<usize>)]`
- `target_subset(&self) -> &[usize]`

**Helper: `compute_closure(x: &HashSet<usize>, fds: &[(Vec<usize>, Vec<usize>)]) -> HashSet<usize>`**
- Start with closure = x.clone()
- Repeat until no change: for each FD (lhs, rhs), if lhs ⊆ closure, add all rhs to closure
- Return closure

**Problem trait:**
```rust
impl Problem for BoyceCoddNormalFormViolation {
    const NAME: &'static str = "BoyceCoddNormalFormViolation";
    type Metric = bool;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.target_subset.len()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        // X = {target_subset[i] : config[i] == 1}
        // Compute X⁺ under F
        // Check: ∃ y,z ∈ A' \ X s.t. y ∈ X⁺ ∧ z ∉ X⁺
        let x: HashSet<usize> = config.iter().enumerate()
            .filter(|(_, &v)| v == 1)
            .map(|(i, _)| self.target_subset[i])
            .collect();
        let closure = Self::compute_closure(&x, &self.functional_deps);
        let outside_x: Vec<usize> = self.target_subset.iter()
            .filter(|a| !x.contains(a))
            .copied()
            .collect();
        let has_in_closure = outside_x.iter().any(|a| closure.contains(a));
        let has_not_in_closure = outside_x.iter().any(|a| !closure.contains(a));
        has_in_closure && has_not_in_closure
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl SatisfactionProblem for BoyceCoddNormalFormViolation {}
```

### Step 1.5: Register variant complexity

```rust
crate::declare_variants! {
    default sat BoyceCoddNormalFormViolation => "2^num_target_attributes * num_target_attributes^2 * num_functional_deps",
}
```

Getter methods: `num_target_attributes()`, `num_functional_deps()` — validated at compile time.

### Step 2: Register in `src/models/misc/mod.rs`

- Add `mod boyce_codd_normal_form_violation;`
- Add `pub use boyce_codd_normal_form_violation::BoyceCoddNormalFormViolation;`
- Add `specs.extend(boyce_codd_normal_form_violation::canonical_model_example_specs());` to `canonical_model_example_specs()`

### Step 3: CLI create support in `problemreductions-cli/src/commands/create.rs`

Add BoyceCoddNormalFormViolation to the import list from `problemreductions::models::misc::`.

Add to the `usage_hint` match:
```
"BoyceCoddNormalFormViolation" => "--n 6 --sets '0,1:2;2:3;3,4:5' --target '0,1,2,3,4,5'"
```

Add to the `create_from_args` match:
- Parse `--n` as num_attributes
- Parse `--sets` as functional dependencies (format: `lhs:rhs;lhs:rhs` where each side is comma-separated indices)
- Parse `--target` as target_subset (comma-separated indices)
- Construct `BoyceCoddNormalFormViolation::new(n, fds, target)`

Since functional dependencies have a unique input format (pairs of attribute sets), use the `--sets` flag for FDs (semicolon-separated, colon between LHS:RHS) and `--target` for target_subset. Use `--n` for num_attributes.

### Step 4: Canonical example in model file

```rust
#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "boyce_codd_normal_form_violation",
        build: || {
            // YES instance: 6 attributes, FDs: {0,1}→{2}, {2}→{3}, {3,4}→{5}
            let problem = BoyceCoddNormalFormViolation::new(
                6,
                vec![
                    (vec![0, 1], vec![2]),
                    (vec![2], vec![3]),
                    (vec![3, 4], vec![5]),
                ],
                vec![0, 1, 2, 3, 4, 5],
            );
            // X={2} witnesses violation: closure={2,3}, y=3∈closure, z=0∉closure
            crate::example_db::specs::satisfaction_example(
                problem,
                vec![vec![0, 0, 1, 0, 0, 0]], // config for X={2}
            )
        },
    }]
}
```

### Step 5: Unit tests in `src/unit_tests/models/misc/boyce_codd_normal_form_violation.rs`

Link from model file:
```rust
#[cfg(test)]
#[path = "../../unit_tests/models/misc/boyce_codd_normal_form_violation.rs"]
mod tests;
```

Test functions (minimum 3):

1. **`test_bcnf_violation_creation`** — construct instances, verify getters
2. **`test_bcnf_violation_evaluate_yes`** — YES instance from issue: X={2} with closure {2,3}, verify evaluate returns true
3. **`test_bcnf_violation_evaluate_no`** — NO instance from issue (cyclic keys): verify all configs return false (or at least specific ones)
4. **`test_bcnf_violation_evaluate_superkey`** — X is a superkey (closure = A'), evaluate returns false
5. **`test_bcnf_violation_evaluate_trivial`** — X has trivial closure (X⁺ = X), evaluate returns false
6. **`test_bcnf_violation_solver`** — use BruteForce::find_satisfying on YES instance, verify it finds a solution; on NO instance, verify None
7. **`test_bcnf_violation_serialization`** — round-trip JSON serialization

Also ensure the unit_tests/models/misc/mod.rs includes the new test module.

### Step 5.5: trait_consistency test

Add `BoyceCoddNormalFormViolation` to the existing trait consistency tests if a pattern file exists, or verify it passes the `example_db` tests via `make test`.

## Batch 2: Paper Entry (Step 6)

### Step 6: Add to `docs/paper/reductions.typ`

**Prerequisites:** Batch 1 must be complete. Run `make paper` to regenerate exports first.

1. Add to `display-name` dictionary:
```typst
"BoyceCoddNormalFormViolation": [Boyce-Codd Normal Form Violation],
```

2. Add `problem-def` entry (place near other misc/database-theory problems):
```typst
#problem-def("BoyceCoddNormalFormViolation")[
  *Instance:* A set $A$ of attribute names, a collection $F$ of functional dependencies on $A$, and a subset $A' subset.eq A$.

  *Question:* Is there a subset $X subset.eq A'$ and two attributes $y, z in A' backslash X$ such that $y in X^+$ but $z in.not X^+$, where $X^+$ is the closure of $X$ under $F$?
][
  A relation satisfies _Boyce-Codd Normal Form_ (BCNF) if every non-trivial functional dependency $X -> Y$ has $X$ as a superkey — that is, $X^+ = A'$.
  This problem asks whether the given attribute subset $A'$ violates BCNF, which is NP-complete by reduction from Hitting Set @BeeriB1979.
]
```

3. Run `make paper` to verify compilation.
