# Variant Trait Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace `type GraphType` and `type Weight` associated types with extensible `fn variant()` method.

**Architecture:** Modify `Problem` trait to use `fn variant() -> Vec<(&'static str, &'static str)>` instead of associated types. Use `std::any::type_name` to extract type names at runtime. Update registry and graph export to use structured variant dict.

**Tech Stack:** Rust, std::any::type_name, serde_json

---

### Task 1: Add variant helper module

**Files:**
- Create: `src/variant.rs`
- Modify: `src/lib.rs`

**Step 1: Create src/variant.rs with short_type_name helper**

```rust
//! Variant attribute utilities.

use std::any::type_name;

/// Extract short type name from full path.
/// e.g., "problemreductions::graph_types::SimpleGraph" -> "SimpleGraph"
pub fn short_type_name<T: 'static>() -> &'static str {
    let full = type_name::<T>();
    full.rsplit("::").next().unwrap_or(full)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_type_name_primitive() {
        assert_eq!(short_type_name::<i32>(), "i32");
        assert_eq!(short_type_name::<f64>(), "f64");
    }

    #[test]
    fn test_short_type_name_struct() {
        struct MyStruct;
        assert_eq!(short_type_name::<MyStruct>(), "MyStruct");
    }
}
```

**Step 2: Add module to src/lib.rs**

Add after other module declarations:
```rust
pub mod variant;
```

**Step 3: Run tests**

Run: `cargo test variant --lib`
Expected: PASS

**Step 4: Commit**

```bash
git add src/variant.rs src/lib.rs
git commit -m "feat: add variant helper module with short_type_name"
```

---

### Task 2: Update Problem trait

**Files:**
- Modify: `src/traits.rs`

**Step 1: Remove GraphType and Weight, add variant() method**

In `src/traits.rs`, update the `Problem` trait:

Remove these lines:
```rust
    /// The graph type this problem operates on.
    type GraphType: GraphMarker;

    /// The weight type for this problem.
    type Weight: NumericWeight;
```

Add this method after `const NAME`:
```rust
    /// Returns attributes describing this problem variant.
    /// Each (key, value) pair describes a variant dimension.
    /// Common keys: "graph", "weight"
    fn variant() -> Vec<(&'static str, &'static str)>;
```

Remove the import of `GraphMarker` from the use statement:
```rust
use crate::graph_types::GraphMarker;
```

Remove `NumericWeight` from the use statement if it's only used for the Weight bound.

**Step 2: Update test problems in traits.rs**

Update `SimpleWeightedProblem` impl:
```rust
impl Problem for SimpleWeightedProblem {
    const NAME: &'static str = "SimpleWeightedProblem";
    type Size = i32;

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", "SimpleGraph"),
            ("weight", "i32"),
        ]
    }
    // ... rest unchanged
}
```

Update `SimpleCsp` impl:
```rust
impl Problem for SimpleCsp {
    const NAME: &'static str = "SimpleCsp";
    type Size = i32;

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", "SimpleGraph"),
            ("weight", "i32"),
        ]
    }
    // ... rest unchanged
}
```

Update `MultiFlavorProblem` impl:
```rust
impl Problem for MultiFlavorProblem {
    const NAME: &'static str = "MultiFlavorProblem";
    type Size = i32;

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", "SimpleGraph"),
            ("weight", "i32"),
        ]
    }
    // ... rest unchanged
}
```

**Step 3: Verify traits.rs compiles**

Run: `cargo check --lib`
Expected: Errors in model files (expected, will fix in next tasks)

**Step 4: Commit**

```bash
git add src/traits.rs
git commit -m "feat: replace GraphType/Weight with variant() in Problem trait"
```

---

### Task 3: Update graph problem models

**Files:**
- Modify: `src/models/graph/independent_set.rs`
- Modify: `src/models/graph/vertex_covering.rs`
- Modify: `src/models/graph/dominating_set.rs`
- Modify: `src/models/graph/matching.rs`
- Modify: `src/models/graph/max_cut.rs`
- Modify: `src/models/graph/coloring.rs`
- Modify: `src/models/graph/maximal_is.rs`

**Step 1: Update independent_set.rs**

Add import at top:
```rust
use crate::variant::short_type_name;
```

In `impl<W> Problem for IndependentSet<W>`, replace:
```rust
    type GraphType = SimpleGraph;
    type Weight = W;
```

With:
```rust
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", "SimpleGraph"),
            ("weight", short_type_name::<W>()),
        ]
    }
```

**Step 2: Update vertex_covering.rs**

Add import at top:
```rust
use crate::variant::short_type_name;
```

In `impl<W> Problem for VertexCovering<W>`, replace:
```rust
    type GraphType = SimpleGraph;
    type Weight = W;
```

With:
```rust
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", "SimpleGraph"),
            ("weight", short_type_name::<W>()),
        ]
    }
```

**Step 3: Update dominating_set.rs**

Add import at top:
```rust
use crate::variant::short_type_name;
```

In `impl<W> Problem for DominatingSet<W>`, replace:
```rust
    type GraphType = SimpleGraph;
    type Weight = W;
```

With:
```rust
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", "SimpleGraph"),
            ("weight", short_type_name::<W>()),
        ]
    }
```

**Step 4: Update matching.rs**

Add import at top:
```rust
use crate::variant::short_type_name;
```

In `impl<W> Problem for Matching<W>`, replace:
```rust
    type GraphType = SimpleGraph;
    type Weight = W;
```

With:
```rust
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", "SimpleGraph"),
            ("weight", short_type_name::<W>()),
        ]
    }
```

**Step 5: Update max_cut.rs**

Add import at top:
```rust
use crate::variant::short_type_name;
```

In `impl<W> Problem for MaxCut<W>`, replace:
```rust
    type GraphType = SimpleGraph;
    type Weight = W;
```

With:
```rust
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", "SimpleGraph"),
            ("weight", short_type_name::<W>()),
        ]
    }
```

**Step 6: Update coloring.rs**

In `impl Problem for Coloring`, replace:
```rust
    type GraphType = SimpleGraph;
    type Weight = i32;
```

With:
```rust
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", "SimpleGraph"),
            ("weight", "i32"),
        ]
    }
```

**Step 7: Update maximal_is.rs**

In `impl Problem for MaximalIS`, replace:
```rust
    type GraphType = SimpleGraph;
    type Weight = i32;
```

With:
```rust
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", "SimpleGraph"),
            ("weight", "i32"),
        ]
    }
```

**Step 8: Verify compilation**

Run: `cargo check --lib`
Expected: More errors (other models still need updating)

**Step 9: Commit**

```bash
git add src/models/graph/
git commit -m "feat: update graph models to use variant()"
```

---

### Task 4: Update satisfiability models

**Files:**
- Modify: `src/models/satisfiability/sat.rs`
- Modify: `src/models/satisfiability/ksat.rs`

**Step 1: Update sat.rs**

Add import at top:
```rust
use crate::variant::short_type_name;
```

In `impl<W> Problem for Satisfiability<W>`, replace:
```rust
    type GraphType = SimpleGraph;
    type Weight = W;
```

With:
```rust
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", "SimpleGraph"),
            ("weight", short_type_name::<W>()),
        ]
    }
```

**Step 2: Update ksat.rs**

Add import at top:
```rust
use crate::variant::short_type_name;
```

In `impl<W, const K: usize> Problem for KSatisfiability<W, K>`, replace:
```rust
    type GraphType = SimpleGraph;
    type Weight = W;
```

With:
```rust
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", "SimpleGraph"),
            ("weight", short_type_name::<W>()),
        ]
    }
```

**Step 3: Commit**

```bash
git add src/models/satisfiability/
git commit -m "feat: update satisfiability models to use variant()"
```

---

### Task 5: Update set models

**Files:**
- Modify: `src/models/set/set_packing.rs`
- Modify: `src/models/set/set_covering.rs`

**Step 1: Update set_packing.rs**

Add import at top:
```rust
use crate::variant::short_type_name;
```

In `impl<W> Problem for SetPacking<W>`, replace:
```rust
    type GraphType = SimpleGraph;
    type Weight = W;
```

With:
```rust
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", "SimpleGraph"),
            ("weight", short_type_name::<W>()),
        ]
    }
```

**Step 2: Update set_covering.rs**

Add import at top:
```rust
use crate::variant::short_type_name;
```

In `impl<W> Problem for SetCovering<W>`, replace:
```rust
    type GraphType = SimpleGraph;
    type Weight = W;
```

With:
```rust
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", "SimpleGraph"),
            ("weight", short_type_name::<W>()),
        ]
    }
```

**Step 3: Commit**

```bash
git add src/models/set/
git commit -m "feat: update set models to use variant()"
```

---

### Task 6: Update optimization models

**Files:**
- Modify: `src/models/optimization/spin_glass.rs`
- Modify: `src/models/optimization/qubo.rs`
- Modify: `src/models/optimization/ilp.rs`

**Step 1: Update spin_glass.rs**

Add import at top:
```rust
use crate::variant::short_type_name;
```

In `impl<W> Problem for SpinGlass<W>`, replace:
```rust
    type GraphType = SimpleGraph;
    type Weight = W;
```

With:
```rust
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", "SimpleGraph"),
            ("weight", short_type_name::<W>()),
        ]
    }
```

**Step 2: Update qubo.rs**

Add import at top:
```rust
use crate::variant::short_type_name;
```

In `impl<W> Problem for QUBO<W>`, replace:
```rust
    type GraphType = SimpleGraph;
    type Weight = W;
```

With:
```rust
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", "SimpleGraph"),
            ("weight", short_type_name::<W>()),
        ]
    }
```

**Step 3: Update ilp.rs**

In `impl Problem for ILP`, replace:
```rust
    type GraphType = SimpleGraph;
    type Weight = f64;
```

With:
```rust
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", "SimpleGraph"),
            ("weight", "f64"),
        ]
    }
```

**Step 4: Commit**

```bash
git add src/models/optimization/
git commit -m "feat: update optimization models to use variant()"
```

---

### Task 7: Update specialized models

**Files:**
- Modify: `src/models/specialized/circuit.rs`
- Modify: `src/models/specialized/factoring.rs`
- Modify: `src/models/specialized/biclique_cover.rs`
- Modify: `src/models/specialized/bmf.rs`
- Modify: `src/models/specialized/paintshop.rs`

**Step 1: Update circuit.rs**

Add import at top:
```rust
use crate::variant::short_type_name;
```

In `impl<W> Problem for CircuitSAT<W>`, replace:
```rust
    type GraphType = SimpleGraph;
    type Weight = W;
```

With:
```rust
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", "SimpleGraph"),
            ("weight", short_type_name::<W>()),
        ]
    }
```

**Step 2: Update factoring.rs**

In `impl Problem for Factoring`, replace:
```rust
    type GraphType = SimpleGraph;
    type Weight = i32;
```

With:
```rust
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", "SimpleGraph"),
            ("weight", "i32"),
        ]
    }
```

**Step 3: Update biclique_cover.rs**

In `impl Problem for BicliqueCover`, replace:
```rust
    type GraphType = SimpleGraph;
    type Weight = i32;
```

With:
```rust
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", "SimpleGraph"),
            ("weight", "i32"),
        ]
    }
```

**Step 4: Update bmf.rs**

In `impl Problem for BMF`, replace:
```rust
    type GraphType = SimpleGraph;
    type Weight = i32;
```

With:
```rust
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", "SimpleGraph"),
            ("weight", "i32"),
        ]
    }
```

**Step 5: Update paintshop.rs**

In `impl Problem for PaintShop`, replace:
```rust
    type GraphType = SimpleGraph;
    type Weight = i32;
```

With:
```rust
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", "SimpleGraph"),
            ("weight", "i32"),
        ]
    }
```

**Step 6: Commit**

```bash
git add src/models/specialized/
git commit -m "feat: update specialized models to use variant()"
```

---

### Task 8: Update template.rs GraphProblem

**Files:**
- Modify: `src/models/graph/template.rs`

**Step 1: Update GraphProblem impl**

In `impl<C, W> Problem for GraphProblem<C, W>`, replace:
```rust
    const NAME: &'static str = C::NAME;
    type GraphType = SimpleGraphMarker;
    type Weight = W;
```

With:
```rust
    const NAME: &'static str = C::NAME;
    type Size = W;

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", "SimpleGraph"),
            ("weight", crate::variant::short_type_name::<W>()),
        ]
    }
```

**Step 2: Commit**

```bash
git add src/models/graph/template.rs
git commit -m "feat: update GraphProblem template to use variant()"
```

---

### Task 9: Update solver test problems

**Files:**
- Modify: `src/solvers/brute_force.rs`

**Step 1: Update test problem implementations**

Find all `impl Problem for` blocks in the test module and replace `type GraphType` and `type Weight` with `fn variant()`.

For `MaxSumProblem`:
```rust
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i32")]
    }
```

For `MinSumProblem`:
```rust
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i32")]
    }
```

For `SelectAtMostOneProblem`:
```rust
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "i32")]
    }
```

For `FloatProblem`:
```rust
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "f64")]
    }
```

For `NearlyEqualProblem`:
```rust
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph"), ("weight", "f64")]
    }
```

**Step 2: Commit**

```bash
git add src/solvers/brute_force.rs
git commit -m "feat: update solver test problems to use variant()"
```

---

### Task 10: Update ReductionEntry in registry

**Files:**
- Modify: `src/rules/registry.rs`

**Step 1: Update ReductionEntry struct**

Replace:
```rust
pub struct ReductionEntry {
    pub source_name: &'static str,
    pub target_name: &'static str,
    pub source_graph: &'static str,
    pub target_graph: &'static str,
    pub source_weighted: bool,
    pub target_weighted: bool,
    pub overhead_fn: fn() -> ReductionOverhead,
}
```

With:
```rust
pub struct ReductionEntry {
    pub source_name: &'static str,
    pub target_name: &'static str,
    pub source_variant: &'static [(&'static str, &'static str)],
    pub target_variant: &'static [(&'static str, &'static str)],
    pub overhead_fn: fn() -> ReductionOverhead,
}
```

**Step 2: Remove variant_id function and related methods**

Delete the `variant_id` function and update/remove `source_variant_id()` and `target_variant_id()` methods.

**Step 3: Update Debug impl**

```rust
impl std::fmt::Debug for ReductionEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReductionEntry")
            .field("source_name", &self.source_name)
            .field("target_name", &self.target_name)
            .field("source_variant", &self.source_variant)
            .field("target_variant", &self.target_variant)
            .field("overhead", &self.overhead())
            .finish()
    }
}
```

**Step 4: Update tests**

Update test `test_reduction_entry_overhead`:
```rust
let entry = ReductionEntry {
    source_name: "TestSource",
    target_name: "TestTarget",
    source_variant: &[("graph", "SimpleGraph"), ("weight", "Unweighted")],
    target_variant: &[("graph", "SimpleGraph"), ("weight", "Unweighted")],
    overhead_fn: || ReductionOverhead::new(vec![("n", poly!(2 * n))]),
};
```

Remove tests: `test_variant_id_base`, `test_variant_id_graph`, `test_variant_id_weighted`, `test_variant_id_both`, `test_entry_variant_ids`.

Update `test_reduction_entries_registered` to not use variant_id.

**Step 5: Commit**

```bash
git add src/rules/registry.rs
git commit -m "feat: update ReductionEntry to use variant slices"
```

---

### Task 11: Update graph.rs JSON export

**Files:**
- Modify: `src/rules/graph.rs`

**Step 1: Update NodeJson struct**

Replace:
```rust
pub struct NodeJson {
    pub id: String,
    pub label: String,
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    pub graph_type: String,
    pub weighted: bool,
}
```

With:
```rust
use std::collections::BTreeMap;

pub struct NodeJson {
    pub name: String,
    pub variant: BTreeMap<String, String>,
    pub category: String,
}
```

**Step 2: Update EdgeJson struct**

Replace:
```rust
pub struct EdgeJson {
    pub source: String,
    pub target: String,
    pub bidirectional: bool,
}
```

With:
```rust
#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct VariantRef {
    pub name: String,
    pub variant: BTreeMap<String, String>,
}

pub struct EdgeJson {
    pub source: VariantRef,
    pub target: VariantRef,
    pub bidirectional: bool,
}
```

**Step 3: Update to_json() method**

Update the `to_json()` method in `ReductionGraph` to build nodes and edges using the new structured format. Use `BTreeMap` to convert variant slices to maps.

**Step 4: Commit**

```bash
git add src/rules/graph.rs
git commit -m "feat: update graph JSON export to structured variant format"
```

---

### Task 12: Update reduction macro

**Files:**
- Modify: `problemreductions-macros/src/lib.rs`

**Step 1: Update ReductionAttrs struct**

Replace graph/weighted attributes with variant:
```rust
struct ReductionAttrs {
    source_variant: Option<Vec<(String, String)>>,
    target_variant: Option<Vec<(String, String)>>,
    overhead: Option<TokenStream2>,
}
```

**Step 2: Update parsing**

Update the Parse impl to handle `source_variant` and `target_variant` as arrays.

**Step 3: Update code generation**

Update `generate_reduction_entry` to output:
```rust
inventory::submit! {
    crate::rules::registry::ReductionEntry {
        source_name: #source_name,
        target_name: #target_name,
        source_variant: &[("graph", "SimpleGraph"), ("weight", "Unweighted")],
        target_variant: &[("graph", "SimpleGraph"), ("weight", "i32")],
        overhead_fn: || { #overhead },
    }
}
```

**Step 4: Commit**

```bash
git add problemreductions-macros/src/lib.rs
git commit -m "feat: update reduction macro for variant slices"
```

---

### Task 13: Update all reduction rule files

**Files:**
- Modify: All files in `src/rules/` that use `inventory::submit!`

**Step 1: Find all reduction registrations**

Run: `grep -l "inventory::submit" src/rules/*.rs`

**Step 2: Update each file**

For each file, update the `inventory::submit!` block to use the new format:
```rust
inventory::submit! {
    ReductionEntry {
        source_name: "SourceProblem",
        target_name: "TargetProblem",
        source_variant: &[("graph", "SimpleGraph"), ("weight", "Unweighted")],
        target_variant: &[("graph", "SimpleGraph"), ("weight", "Unweighted")],
        overhead_fn: || ReductionOverhead::new(...),
    }
}
```

**Step 3: Commit**

```bash
git add src/rules/
git commit -m "feat: update all reduction registrations to variant format"
```

---

### Task 14: Remove GraphMarker::NAME

**Files:**
- Modify: `src/graph_types.rs`

**Step 1: Remove NAME from GraphMarker trait**

Remove:
```rust
    const NAME: &'static str;
```

**Step 2: Remove NAME from all impls**

Remove the `const NAME` line from:
- `impl GraphMarker for SimpleGraph`
- `impl GraphMarker for PlanarGraph`
- `impl GraphMarker for UnitDiskGraph`
- `impl GraphMarker for BipartiteGraph`

**Step 3: Update any code that uses GraphMarker::NAME**

Search for usages and replace with `short_type_name::<G>()` calls.

**Step 4: Commit**

```bash
git add src/graph_types.rs
git commit -m "feat: remove NAME from GraphMarker trait"
```

---

### Task 15: Run full test suite and fix issues

**Step 1: Run cargo check**

Run: `cargo check --all-features`
Fix any compilation errors.

**Step 2: Run tests**

Run: `cargo test --all-features`
Fix any test failures.

**Step 3: Run clippy**

Run: `cargo clippy --all-features`
Fix any warnings.

**Step 4: Commit fixes**

```bash
git add -A
git commit -m "fix: resolve compilation and test issues"
```

---

### Task 16: Regenerate reduction graph

**Step 1: Run export-graph**

Run: `make export-graph`

**Step 2: Verify JSON format**

Check `docs/paper/reduction_graph.json` has the new structured format.

**Step 3: Commit**

```bash
git add docs/paper/reduction_graph.json
git commit -m "chore: regenerate reduction graph with new variant format"
```

---

### Task 17: Final verification

**Step 1: Run full CI checks**

Run: `make test clippy`

**Step 2: Verify all tests pass**

Expected: All tests pass, no clippy warnings.
