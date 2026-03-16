# Plan: Add AdditionalKey Model (#445)

## Overview

Add the `AdditionalKey` satisfaction problem from relational database theory (Garey & Johnson A4 SR27). Given a set of attributes, functional dependencies, a relation scheme, and known keys, determine whether there exists a candidate key not in the known set.

**Problem type:** Satisfaction (`Metric = bool`)
**Category:** `misc` (unique input structure — relational database theory)
**Type parameters:** None
**Associated rules:** R121 HittingSet → AdditionalKey (inbound only — this model will be an orphan until that rule is implemented)

## Information Checklist

| # | Item | Value |
|---|------|-------|
| 1 | Problem name | `AdditionalKey` |
| 2 | Mathematical definition | Given attribute set A, functional dependencies F on A, relation R ⊆ A, and known keys K for ⟨R,F⟩, determine if R has a candidate key not in K |
| 3 | Problem type | Satisfaction (decision) |
| 4 | Type parameters | None |
| 5 | Struct fields | `num_attributes: usize`, `dependencies: Vec<(Vec<usize>, Vec<usize>)>`, `relation_attrs: Vec<usize>`, `known_keys: Vec<Vec<usize>>` |
| 6 | Configuration space | `vec![2; relation_attrs.len()]` — binary selection over relation attributes |
| 7 | Feasibility check | Closure of selected attrs under F covers all relation_attrs, selected set is minimal (no proper subset also covers), and selected set is not in known_keys |
| 8 | Objective function | N/A (satisfaction: returns bool) |
| 9 | Best known exact algorithm | Brute-force enumeration: O(2^num_relation_attrs × num_dependencies × num_attributes) — Beeri & Bernstein (1979) |
| 10 | Solving strategy | BruteForce works directly |
| 11 | Category | `misc` |
| 12 | Expected outcome | Instance 1 (YES): attrs {0,2} is an additional key. Instance 2 (NO): only key {0} is already known |

## Batch 1: Implementation (Steps 1–5.5)

### Step 1: Create model file `src/models/misc/additional_key.rs`

Follow `SubsetSum` as the reference satisfaction problem pattern.

**Schema registration:**
```rust
inventory::submit! {
    ProblemSchemaEntry {
        name: "AdditionalKey",
        display_name: "Additional Key",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Determine whether a relational schema has a candidate key not in a given set",
        fields: &[
            FieldInfo { name: "num_attributes", type_name: "usize", description: "Number of attributes in A" },
            FieldInfo { name: "dependencies", type_name: "Vec<(Vec<usize>, Vec<usize>)>", description: "Functional dependencies F; each (lhs, rhs)" },
            FieldInfo { name: "relation_attrs", type_name: "Vec<usize>", description: "Relation scheme attributes R ⊆ A" },
            FieldInfo { name: "known_keys", type_name: "Vec<Vec<usize>>", description: "Known candidate keys K" },
        ],
    }
}
```

**Struct:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdditionalKey {
    num_attributes: usize,
    dependencies: Vec<(Vec<usize>, Vec<usize>)>,
    relation_attrs: Vec<usize>,
    known_keys: Vec<Vec<usize>>,
}
```

**Constructor:** `new(num_attributes, dependencies, relation_attrs, known_keys)` with validation:
- All attribute indices < num_attributes
- relation_attrs elements are unique and sorted
- known_keys entries are sorted for consistent comparison

**Size getters:**
- `num_attributes() -> usize`
- `num_dependencies() -> usize` (returns `dependencies.len()`)
- `num_relation_attrs() -> usize` (returns `relation_attrs.len()`)
- `num_known_keys() -> usize` (returns `known_keys.len()`)

**Core algorithm — closure computation:**
```rust
fn compute_closure(&self, attrs: &HashSet<usize>) -> HashSet<usize> {
    let mut closure = attrs.clone();
    let mut changed = true;
    while changed {
        changed = false;
        for (lhs, rhs) in &self.dependencies {
            if lhs.iter().all(|a| closure.contains(a)) {
                for &a in rhs {
                    if closure.insert(a) {
                        changed = true;
                    }
                }
            }
        }
    }
    closure
}
```

**Problem trait:**
```rust
impl Problem for AdditionalKey {
    const NAME: &'static str = "AdditionalKey";
    type Metric = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.relation_attrs.len()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        // 1. Check config length and binary values
        // 2. Build selected attribute set from config
        // 3. Compute closure under FDs
        // 4. Check closure covers all relation_attrs
        // 5. Check minimality: no proper subset also has full closure
        // 6. Check selected set is not in known_keys
    }
}

impl SatisfactionProblem for AdditionalKey {}
```

**Variant declaration:**
```rust
crate::declare_variants! {
    default sat AdditionalKey => "2^num_relation_attrs * num_dependencies * num_attributes",
}
```

### Step 2: Register in module system

1. **`src/models/misc/mod.rs`**: Add `mod additional_key;` and `pub use additional_key::AdditionalKey;`
2. **`src/models/mod.rs`**: Add `AdditionalKey` to the `misc` re-export line

### Step 3: CLI registration

1. **`problemreductions-cli/src/commands/create.rs`**: Add match arm for `"AdditionalKey"` that parses `--dependencies`, `--relation-attrs`, `--known-keys`, and `--num-attributes` flags. Since this problem has a unique schema, it needs custom argument parsing.

2. **`problemreductions-cli/src/cli.rs`**: Add CLI flags if not already present:
   - `--num-attributes` (usize)
   - `--dependencies` (string, e.g., "0,1:2,3;2,3:4,5" for {0,1}→{2,3} and {2,3}→{4,5})
   - `--relation-attrs` (comma-separated usize list)
   - `--known-keys` (string, e.g., "0,1;2,3" for [{0,1}, {2,3}])

3. **Help text**: Add entry to "Flags by problem type" table

### Step 4: Add canonical model example

In `src/models/misc/additional_key.rs`, add:
```rust
#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "additional_key",
        build: || {
            let problem = AdditionalKey::new(
                6,
                vec![
                    (vec![0, 1], vec![2, 3]),
                    (vec![2, 3], vec![4, 5]),
                    (vec![4, 5], vec![0, 1]),
                    (vec![0, 2], vec![3]),
                    (vec![3, 5], vec![1]),
                ],
                vec![0, 1, 2, 3, 4, 5],
                vec![vec![0, 1], vec![2, 3], vec![4, 5]],
            );
            // Config for key {0, 2}: [1, 0, 1, 0, 0, 0]
            crate::example_db::specs::satisfaction_example(
                problem,
                vec![vec![1, 0, 1, 0, 0, 0]],
            )
        },
    }]
}
```

Register in `src/models/misc/mod.rs` `canonical_model_example_specs()`.

### Step 5: Write unit tests

Create `src/unit_tests/models/misc/additional_key.rs` with:

1. **`test_additional_key_creation`**: Construct instance, verify getters and dims
2. **`test_additional_key_evaluate_satisfying`**: Instance 1 — verify {0,2} is a valid additional key
3. **`test_additional_key_evaluate_unsatisfying`**: Instance 2 — verify no additional key exists
4. **`test_additional_key_evaluate_non_minimal`**: Verify a superset of a key returns false
5. **`test_additional_key_evaluate_known_key`**: Verify a known key returns false
6. **`test_additional_key_evaluate_not_a_key`**: Verify a non-key (closure ≠ R) returns false
7. **`test_additional_key_wrong_config_length`**: Wrong-length config returns false
8. **`test_additional_key_invalid_variable_value`**: Config with value ≥ 2 returns false
9. **`test_additional_key_brute_force`**: BruteForce finds a satisfying solution
10. **`test_additional_key_brute_force_all`**: All satisfying solutions are valid
11. **`test_additional_key_serialization`**: Round-trip serde test
12. **`test_additional_key_paper_example`**: Verify the paper example instance (same as canonical example)

Link via `#[cfg(test)] #[path = "..."] mod tests;` at the bottom of the model file.

### Step 5.5: Add trait_consistency entry

In `src/unit_tests/trait_consistency.rs`:
- Add `check_problem_trait(...)` call for `AdditionalKey` with the small Instance 2 (3 attributes)

## Batch 2: Paper Entry (Step 6)

### Step 6: Document in paper

**File:** `docs/paper/reductions.typ`

1. **Add display name:**
```typst
"AdditionalKey": [Additional Key],
```

2. **Add problem-def:**
```typst
#problem-def("AdditionalKey")[
  Given a set $A$ of attribute names, a collection $F$ of functional dependencies on $A$,
  a subset $R subset.eq A$, and a set $K$ of candidate keys for the relational scheme $angle.l R, F angle.r$,
  determine whether there exists a subset $R' subset.eq R$ such that $R' in.not K$,
  $(R', R) in F^*$, and for no $R'' subset.neq R'$ is $(R'', R) in F^*$.
][
  A classical NP-complete problem from relational database theory @beeri1979.
  The problem is central to database normalization: enumerating all candidate keys
  is necessary to verify Boyce-Codd Normal Form (BCNF), and the NP-completeness
  of Additional Key implies that BCNF testing is intractable in general.
  The best known exact algorithm is brute-force enumeration of all $2^(|R|)$ subsets,
  checking each for the key property via closure computation under Armstrong's axioms.
  #footnote[No algorithm improving on brute-force is known for the Additional Key problem.]

  *Example.* Let $A = {0, 1, 2, 3, 4, 5}$ with functional dependencies
  ${0,1} -> {2,3}$, ${2,3} -> {4,5}$, ${4,5} -> {0,1}$,
  ${0,2} -> {3}$, ${3,5} -> {1}$, relation $R = A$,
  and known keys $K = {{0,1}, {2,3}, {4,5}}$.
  The subset ${0,2}$ is an additional key: its closure under $F$ reaches all of $A$
  via ${0,2} -> {3} -> {4,5} -> {0,1}$, it is minimal, and ${0,2} in.not K$.
]
```

3. **Add BibTeX entry** for Beeri & Bernstein (1979) if not already present.

4. **Build and verify:** `make paper`
