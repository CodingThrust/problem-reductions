# Plan: [Model] ConjunctiveBooleanQuery (#449)

## Overview

Add the **Conjunctive Boolean Query** (CBQ) problem model — a satisfaction problem from database theory (Garey & Johnson A4 SR31). Given a finite domain D, a collection of relations R, and an existentially quantified conjunctive query Q, determine whether Q is true over R and D.

This is equivalent to the Constraint Satisfaction Problem (CSP) and the Graph Homomorphism problem. NP-complete via reduction from CLIQUE (Chandra & Merlin, 1977).

**Category:** `misc/` (unique input structure: domain + relations + conjunctive query)

## Information Checklist

| # | Item | Value |
|---|------|-------|
| 1 | Problem name | `ConjunctiveBooleanQuery` |
| 2 | Mathematical definition | INSTANCE: Finite domain D, relations R={R_1,...,R_m}, conjunctive query Q = (∃y_1,...,y_l)(A_1 ∧ ... ∧ A_r). QUESTION: Is Q true? |
| 3 | Problem type | Satisfaction (`Metric = bool`) |
| 4 | Type parameters | None |
| 5 | Struct fields | `domain_size: usize`, `relations: Vec<Relation>`, `num_variables: usize`, `conjuncts: Vec<(usize, Vec<QueryArg>)>` |
| 6 | Configuration space | `vec![domain_size; num_variables]` — l variables each over domain D |
| 7 | Feasibility check | For each conjunct (rel_idx, args), substitute variables from config and check tuple membership in the relation |
| 8 | Objective function | N/A (satisfaction) |
| 9 | Best known exact | Brute-force O(|D|^l * r * max_arity). No substantially faster general algorithm known. |
| 10 | Solving strategy | BruteForce (enumerate all |D|^l assignments) |
| 11 | Category | `misc` |
| 12 | Expected outcome | Example: D={0..5}, 2 relations (binary+ternary), query with 2 variables, 3 conjuncts → TRUE, satisfying assignment y_1=0, y_2=1 |

## Batch 1: Implementation (Steps 1-5.5)

### Task 1.1: Create model file `src/models/misc/conjunctive_boolean_query.rs`

Define companion types and the main struct:

```rust
/// A relation with fixed arity and a set of tuples.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub arity: usize,
    pub tuples: Vec<Vec<usize>>,
}

/// An argument in a conjunctive query: either a bound variable or a domain constant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryArg {
    Variable(usize),
    Constant(usize),
}

/// The Conjunctive Boolean Query problem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConjunctiveBooleanQuery {
    domain_size: usize,
    relations: Vec<Relation>,
    num_variables: usize,
    conjuncts: Vec<(usize, Vec<QueryArg>)>,
}
```

**Constructor:** `new(domain_size, relations, num_variables, conjuncts)` with validation:
- All relation tuples have correct arity
- All tuple entries < domain_size
- All conjunct relation indices are valid
- All conjunct args reference valid variable indices or constants < domain_size
- Conjunct argument count matches relation arity

**Getter methods:** `domain_size()`, `num_relations()`, `num_variables()`, `num_conjuncts()`, `relations()`, `conjuncts()`

**Problem trait impl:**
- `NAME = "ConjunctiveBooleanQuery"`
- `type Metric = bool`
- `variant() -> crate::variant_params![]`
- `dims() -> vec![self.domain_size; self.num_variables]`
- `evaluate(config)`: for each conjunct, resolve args (Variable → config[idx], Constant → value), check if resulting tuple is in the relation

**SatisfactionProblem impl:** marker trait

**`declare_variants!`:**
```rust
crate::declare_variants! {
    default sat ConjunctiveBooleanQuery => "domain_size ^ num_variables",
}
```

**ProblemSchemaEntry:**
```rust
inventory::submit! {
    ProblemSchemaEntry {
        name: "ConjunctiveBooleanQuery",
        display_name: "Conjunctive Boolean Query",
        aliases: &["CBQ"],
        dimensions: &[],
        module_path: module_path!(),
        description: "Evaluate a conjunctive Boolean query over a relational database",
        fields: &[
            FieldInfo { name: "domain_size", type_name: "usize", description: "Size of the finite domain D" },
            FieldInfo { name: "relations", type_name: "Vec<Relation>", description: "Collection of relations R" },
            FieldInfo { name: "num_variables", type_name: "usize", description: "Number of existentially quantified variables" },
            FieldInfo { name: "conjuncts", type_name: "Vec<(usize, Vec<QueryArg>)>", description: "Query conjuncts: (relation_index, arguments)" },
        ],
    }
}
```

### Task 1.2: Register in module tree

1. **`src/models/misc/mod.rs`:** Add `mod conjunctive_boolean_query;` and `pub use conjunctive_boolean_query::{ConjunctiveBooleanQuery, Relation as CbqRelation, QueryArg};`
2. **`src/models/mod.rs`:** Add to misc re-exports
3. **`src/lib.rs`/prelude:** Add ConjunctiveBooleanQuery to prelude if needed

### Task 1.3: CLI registration

1. **`problemreductions-cli/src/problem_name.rs`:** Add alias `"cbq" => "ConjunctiveBooleanQuery"` (CBQ is not a well-established abbreviation, so only add lowercase pass-through; the `aliases` in ProblemSchemaEntry handles `CBQ`)
2. **`problemreductions-cli/src/commands/create.rs`:**
   - Add CLI flags to `CreateArgs` in `cli.rs`: `--domain-size`, `--relations`, `--conjuncts`
   - Add match arm for `"ConjunctiveBooleanQuery"` that parses these flags
   - Add to `help_data_flags()` table
   - Update `all_data_flags_empty()` if new flags added
3. **`problemreductions-cli/src/cli.rs`:** Add the new flag fields

### Task 1.4: Add canonical model example to example_db

Add `canonical_model_example_specs()` to the model file (feature-gated with `example-db`):

Use the issue's example instance:
- D = {0,...,5}, R_1 (arity 2): {(0,3),(1,3),(2,4),(3,4),(4,5)}, R_2 (arity 3): {(0,1,5),(1,2,5),(2,3,4),(0,4,3)}
- Query: (∃y_1,y_2)(R_1(y_1,3) ∧ R_1(y_2,3) ∧ R_2(y_1,y_2,5))
- But keep it small enough for brute force (6^2 = 36 configs — fine)

Register in `src/models/misc/mod.rs`'s `canonical_model_example_specs()`.

### Task 1.5: Write unit tests `src/unit_tests/models/misc/conjunctive_boolean_query.rs`

Required tests (≥3):
- `test_conjunctivebooleanquery_basic` — create instance, verify dims, getters, NAME
- `test_conjunctivebooleanquery_evaluate_yes` — verify satisfying assignment y_1=0, y_2=1
- `test_conjunctivebooleanquery_evaluate_no` — verify non-satisfying assignment
- `test_conjunctivebooleanquery_out_of_range` — variable value ≥ domain_size returns false
- `test_conjunctivebooleanquery_brute_force` — BruteForce finds satisfying assignment
- `test_conjunctivebooleanquery_unsatisfiable` — instance with no solution
- `test_conjunctivebooleanquery_serialization` — round-trip serde JSON
- `test_conjunctivebooleanquery_paper_example` — same instance as paper, verify expected solution and count

Link test file via `#[cfg(test)] #[path = "../../unit_tests/models/misc/conjunctive_boolean_query.rs"] mod tests;`

### Task 1.6: Verify build and tests

```bash
make check  # fmt + clippy + test
```

## Batch 2: Paper Entry (Step 6)

### Task 2.1: Write paper entry in `docs/paper/reductions.typ`

1. Add to `display-name` dict: `"ConjunctiveBooleanQuery": [Conjunctive Boolean Query]`
2. Add `#problem-def("ConjunctiveBooleanQuery")[definition][body]` with:
   - Formal definition: domain D, relations R, conjunctive query Q
   - Background: classical in database theory, equivalent to CSP and graph homomorphism
   - Best known algorithm: brute-force O(|D|^l), cite Chandra & Merlin 1977
   - Example with CeTZ visualization: show the relations as tables, the query structure, and the satisfying assignment
   - Evaluation: verify the three conjuncts on (y_1=0, y_2=1)
3. Build: `make paper`

## Dependencies

- Batch 2 depends on Batch 1 (needs exports from compiled code)
- No external model dependencies (this is a standalone model)
