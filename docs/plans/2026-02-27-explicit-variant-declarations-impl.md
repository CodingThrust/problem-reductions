# Explicit Variant Declarations Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Make problem variants first-class citizens with explicit declarations and per-variant time complexity metadata.

**Architecture:** New `VariantEntry` inventory + `declare_variants!` macro in model files. `DeclaredVariant` marker trait enables compile-time checking in `#[reduction]`. `ReductionGraph` builds nodes from `VariantEntry` instead of inferring them from edges.

**Tech Stack:** Rust, inventory crate, macro_rules!, proc_macro (existing `#[reduction]`), petgraph

---

### Task 1: Add DeclaredVariant trait

**Files:**
- Modify: `src/traits.rs`

**Step 1: Add the marker trait**

At the end of `src/traits.rs` (before the `#[cfg(test)]` block), add:

```rust
/// Marker trait for explicitly declared problem variants.
///
/// Implemented automatically by [`declare_variants!`] for each concrete type.
/// The [`#[reduction]`] proc macro checks this trait at compile time to ensure
/// all reduction source/target types have been declared.
pub trait DeclaredVariant {}
```

**Step 2: Build**

Run: `cargo build`
Expected: PASS (trait is unused so far)

**Step 3: Commit**

```bash
git add src/traits.rs
git commit -m "feat: add DeclaredVariant marker trait"
```

---

### Task 2: Add VariantEntry struct and inventory

**Files:**
- Create: `src/registry/variant.rs`
- Modify: `src/registry/mod.rs`

**Step 1: Create the variant entry module**

Create `src/registry/variant.rs`:

```rust
//! Explicit variant registration via inventory.

/// A registered problem variant entry.
///
/// Submitted by [`declare_variants!`] for each concrete problem type.
/// The reduction graph uses these entries to build nodes with complexity metadata.
pub struct VariantEntry {
    /// Problem name (from `Problem::NAME`).
    pub name: &'static str,
    /// Function returning variant key-value pairs (from `Problem::variant()`).
    pub variant_fn: fn() -> Vec<(&'static str, &'static str)>,
    /// Worst-case time complexity expression (e.g., `"2^num_vertices"`).
    pub complexity: &'static str,
}

impl VariantEntry {
    /// Get the variant by calling the function.
    pub fn variant(&self) -> Vec<(&'static str, &'static str)> {
        (self.variant_fn)()
    }
}

impl std::fmt::Debug for VariantEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VariantEntry")
            .field("name", &self.name)
            .field("variant", &self.variant())
            .field("complexity", &self.complexity)
            .finish()
    }
}

inventory::collect!(VariantEntry);
```

**Step 2: Export from registry module**

In `src/registry/mod.rs`, add the module declaration and re-export:

```rust
pub mod variant;
pub use variant::VariantEntry;
```

**Step 3: Build**

Run: `cargo build`
Expected: PASS

**Step 4: Commit**

```bash
git add src/registry/variant.rs src/registry/mod.rs
git commit -m "feat: add VariantEntry inventory struct"
```

---

### Task 3: Create declare_variants! macro

**Files:**
- Modify: `src/variant.rs` (where `variant_params!` is defined)

**Step 1: Add the macro**

At the end of `src/variant.rs`, add:

```rust
/// Declare explicit problem variants with per-variant complexity metadata.
///
/// Each entry generates:
/// 1. A `DeclaredVariant` trait impl for compile-time checking
/// 2. A `VariantEntry` inventory submission for runtime graph building
///
/// # Example
///
/// ```ignore
/// declare_variants! {
///     MaximumIndependentSet<SimpleGraph, i32>   => "2^num_vertices",
///     MaximumIndependentSet<KingsSubgraph, i32> => "2^num_vertices",
/// }
/// ```
#[macro_export]
macro_rules! declare_variants {
    ($($ty:ty => $complexity:expr),+ $(,)?) => {
        $(
            impl $crate::traits::DeclaredVariant for $ty {}

            $crate::inventory::submit! {
                $crate::registry::VariantEntry {
                    name: <$ty as $crate::traits::Problem>::NAME,
                    variant_fn: || <$ty as $crate::traits::Problem>::variant(),
                    complexity: $complexity,
                }
            }
        )+
    };
}
```

**Step 2: Check inventory re-export**

Verify that `inventory` is re-exported from the main crate. Check `src/lib.rs` for `pub use inventory;` or similar. If not present, add:

```rust
pub use inventory;
```

**Step 3: Build**

Run: `cargo build`
Expected: PASS

**Step 4: Commit**

```bash
git add src/variant.rs src/lib.rs
git commit -m "feat: add declare_variants! macro"
```

---

### Task 4: Add declare_variants! to graph model files

**Files (9 model files):**
- Modify: `src/models/graph/maximum_independent_set.rs`
- Modify: `src/models/graph/minimum_vertex_cover.rs`
- Modify: `src/models/graph/maximum_clique.rs`
- Modify: `src/models/graph/minimum_dominating_set.rs`
- Modify: `src/models/graph/maximum_matching.rs`
- Modify: `src/models/graph/traveling_salesman.rs`
- Modify: `src/models/graph/max_cut.rs`
- Modify: `src/models/graph/kcoloring.rs`
- Modify: `src/models/graph/maximal_is.rs` (optional — no reductions)

**Step 1: Add declarations to maximum_independent_set.rs**

Add at the end of the file (before `#[cfg(test)]`):

```rust
declare_variants! {
    MaximumIndependentSet<SimpleGraph, i32>       => "2^num_vertices",
    MaximumIndependentSet<KingsSubgraph, i32>     => "2^num_vertices",
    MaximumIndependentSet<TriangularSubgraph, i32> => "2^num_vertices",
    MaximumIndependentSet<UnitDiskGraph, i32>     => "2^num_vertices",
}
```

Ensure the geometry graph type imports are present:
```rust
use crate::graphs::{KingsSubgraph, TriangularSubgraph, UnitDiskGraph};
```

**Step 2: Add declarations to minimum_vertex_cover.rs**

```rust
declare_variants! {
    MinimumVertexCover<SimpleGraph, i32> => "2^num_vertices",
}
```

**Step 3: Add declarations to maximum_clique.rs**

```rust
declare_variants! {
    MaximumClique<SimpleGraph, i32> => "2^num_vertices",
}
```

**Step 4: Add declarations to minimum_dominating_set.rs**

```rust
declare_variants! {
    MinimumDominatingSet<SimpleGraph, i32> => "2^num_vertices",
}
```

**Step 5: Add declarations to maximum_matching.rs**

```rust
declare_variants! {
    MaximumMatching<SimpleGraph, i32> => "2^num_vertices",
}
```

**Step 6: Add declarations to traveling_salesman.rs**

```rust
declare_variants! {
    TravelingSalesman<SimpleGraph, i32> => "num_vertices!",
}
```

**Step 7: Add declarations to max_cut.rs**

```rust
declare_variants! {
    MaxCut<SimpleGraph, i32> => "2^num_vertices",
}
```

**Step 8: Add declarations to kcoloring.rs**

```rust
use crate::graphs::SimpleGraph;
use crate::variant::{KN, K2, K3, K4, K5};

declare_variants! {
    KColoring<KN, SimpleGraph> => "k^num_vertices",
    KColoring<K2, SimpleGraph> => "2^num_vertices",
    KColoring<K3, SimpleGraph> => "3^num_vertices",
    KColoring<K4, SimpleGraph> => "4^num_vertices",
    KColoring<K5, SimpleGraph> => "5^num_vertices",
}
```

**Step 9: Build**

Run: `cargo build`
Expected: PASS

**Step 10: Commit**

```bash
git add src/models/graph/
git commit -m "feat: add declare_variants! to graph model files"
```

---

### Task 5: Add declare_variants! to optimization, satisfiability, set, and specialized model files

**Files (9 model files):**
- Modify: `src/models/optimization/qubo.rs`
- Modify: `src/models/optimization/spin_glass.rs`
- Modify: `src/models/optimization/ilp.rs`
- Modify: `src/models/satisfiability/sat.rs`
- Modify: `src/models/satisfiability/ksat.rs`
- Modify: `src/models/set/maximum_set_packing.rs`
- Modify: `src/models/set/minimum_set_covering.rs`
- Modify: `src/models/specialized/circuit.rs`
- Modify: `src/models/specialized/factoring.rs`

**Step 1: qubo.rs**

```rust
declare_variants! {
    QUBO<f64> => "2^num_vars",
}
```

**Step 2: spin_glass.rs**

```rust
use crate::graphs::SimpleGraph;

declare_variants! {
    SpinGlass<SimpleGraph, i32> => "2^num_vertices",
    SpinGlass<SimpleGraph, f64> => "2^num_vertices",
}
```

**Step 3: ilp.rs**

```rust
declare_variants! {
    ILP => "exp(num_variables)",
}
```

**Step 4: sat.rs**

```rust
declare_variants! {
    Satisfiability => "2^num_variables",
}
```

**Step 5: ksat.rs**

```rust
use crate::variant::{KN, K2, K3};

declare_variants! {
    KSatisfiability<KN> => "2^num_variables",
    KSatisfiability<K2> => "2^num_variables",
    KSatisfiability<K3> => "2^num_variables",
}
```

**Step 6: maximum_set_packing.rs**

```rust
declare_variants! {
    MaximumSetPacking<i32> => "2^num_sets",
    MaximumSetPacking<f64> => "2^num_sets",
}
```

**Step 7: minimum_set_covering.rs**

```rust
declare_variants! {
    MinimumSetCovering<i32> => "2^num_sets",
}
```

**Step 8: circuit.rs**

```rust
declare_variants! {
    CircuitSAT => "2^num_inputs",
}
```

**Step 9: factoring.rs**

```rust
declare_variants! {
    Factoring => "exp(sqrt(num_bits))",
}
```

**Step 10: Build and test**

Run: `cargo build && cargo test`
Expected: PASS

**Step 11: Commit**

```bash
git add src/models/optimization/ src/models/satisfiability/ src/models/set/ src/models/specialized/
git commit -m "feat: add declare_variants! to remaining model files"
```

---

### Task 6: Update #[reduction] proc macro to check DeclaredVariant

**Files:**
- Modify: `problemreductions-macros/src/lib.rs`

**Step 1: Add DeclaredVariant assertion to generate_reduction_entry()**

In the `generate_reduction_entry` function, after the `inventory::submit!` block, add a compile-time assertion. Find the section that builds the final `output` tokens (around line 260-282) and append:

```rust
// After the inventory::submit! block, add:
let declared_check = quote! {
    const _: () = {
        fn _assert_declared_variant<T: crate::traits::DeclaredVariant>() {}
        _assert_declared_variant::<#source_type>();
        _assert_declared_variant::<#target_type>();
    };
};
```

Include `declared_check` in the final output token stream.

**Step 2: Build**

Run: `cargo build`
Expected: PASS (all variants are already declared from Tasks 4-5)

**Step 3: Verify enforcement works**

Temporarily comment out one variant from a `declare_variants!` call (e.g., remove `MaximumIndependentSet<KingsSubgraph, i32>` from MIS), then build:

Run: `cargo build 2>&1 | head -20`
Expected: Compile error mentioning `DeclaredVariant` not implemented for `MaximumIndependentSet<KingsSubgraph, i32>`

Restore the commented-out variant.

**Step 4: Commit**

```bash
git add problemreductions-macros/src/lib.rs
git commit -m "feat: #[reduction] now checks DeclaredVariant at compile time"
```

---

### Task 7: Update ReductionGraph to build nodes from VariantEntry

**Files:**
- Modify: `src/rules/graph.rs`

**Step 1: Write a test for variant complexity in the graph**

In `src/unit_tests/rules/graph.rs`, add:

```rust
#[test]
fn test_variant_entry_complexity_available() {
    // VariantEntry inventory should have entries with complexity info
    let entries: Vec<_> = inventory::iter::<crate::registry::VariantEntry>.into_iter().collect();
    assert!(!entries.is_empty(), "VariantEntry inventory should not be empty");

    // Check MIS has a variant with complexity
    let mis_entry = entries.iter().find(|e| e.name == "MaximumIndependentSet");
    assert!(mis_entry.is_some(), "MIS should have a VariantEntry");
    assert!(!mis_entry.unwrap().complexity.is_empty(), "complexity should not be empty");
}
```

**Step 2: Run test**

Run: `cargo test test_variant_entry_complexity_available`
Expected: PASS (VariantEntry submissions exist from Tasks 4-5)

**Step 3: Add complexity field to VariantNode**

In `src/rules/graph.rs`, update `VariantNode`:

```rust
#[derive(Debug, Clone)]
struct VariantNode {
    name: &'static str,
    variant: BTreeMap<String, String>,
    complexity: &'static str,
}
```

**Step 4: Update ReductionGraph::new() to build nodes from VariantEntry first**

Replace the node-building logic in `new()`. The new approach:

1. First pass: create nodes from `VariantEntry` inventory
2. Second pass: create edges from `ReductionEntry` inventory (nodes must already exist)

```rust
pub fn new() -> Self {
    let mut graph = DiGraph::new();
    let mut nodes: Vec<VariantNode> = Vec::new();
    let mut node_index: HashMap<VariantRef, NodeIndex> = HashMap::new();
    let mut name_to_nodes: HashMap<&'static str, Vec<NodeIndex>> = HashMap::new();

    // Helper to ensure a variant node exists
    let mut ensure_node = |name: &'static str,
                           variant: BTreeMap<String, String>,
                           complexity: &'static str,
                           nodes: &mut Vec<VariantNode>,
                           graph: &mut DiGraph<usize, ReductionEdgeData>,
                           node_index: &mut HashMap<VariantRef, NodeIndex>,
                           name_to_nodes: &mut HashMap<&'static str, Vec<NodeIndex>>|
     -> NodeIndex {
        let vref = VariantRef {
            name: name.to_string(),
            variant: variant.clone(),
        };
        if let Some(&idx) = node_index.get(&vref) {
            idx
        } else {
            let node_id = nodes.len();
            nodes.push(VariantNode { name, variant, complexity });
            let idx = graph.add_node(node_id);
            node_index.insert(vref, idx);
            name_to_nodes.entry(name).or_default().push(idx);
            idx
        }
    };

    // Phase 1: Build nodes from VariantEntry inventory
    for entry in inventory::iter::<crate::registry::VariantEntry> {
        let variant = Self::variant_to_map(&entry.variant());
        ensure_node(
            entry.name,
            variant,
            entry.complexity,
            &mut nodes,
            &mut graph,
            &mut node_index,
            &mut name_to_nodes,
        );
    }

    // Phase 2: Build edges from ReductionEntry inventory
    for entry in inventory::iter::<ReductionEntry> {
        let source_variant = Self::variant_to_map(&entry.source_variant());
        let target_variant = Self::variant_to_map(&entry.target_variant());

        // Nodes should already exist from Phase 1 (enforced by #[reduction] compile check).
        // Fall back to creating them with empty complexity for backwards compatibility.
        let src_idx = ensure_node(
            entry.source_name,
            source_variant,
            "",
            &mut nodes,
            &mut graph,
            &mut node_index,
            &mut name_to_nodes,
        );
        let dst_idx = ensure_node(
            entry.target_name,
            target_variant,
            "",
            &mut nodes,
            &mut graph,
            &mut node_index,
            &mut name_to_nodes,
        );

        let overhead = entry.overhead();
        if graph.find_edge(src_idx, dst_idx).is_none() {
            graph.add_edge(
                src_idx,
                dst_idx,
                ReductionEdgeData {
                    overhead,
                    reduce_fn: entry.reduce_fn,
                },
            );
        }
    }

    Self { graph, nodes, name_to_nodes }
}
```

**Step 5: Add complexity getter to ReductionGraph**

```rust
/// Get the complexity expression for a specific variant.
pub fn variant_complexity(
    &self,
    name: &str,
    variant: &BTreeMap<String, String>,
) -> Option<&'static str> {
    let idx = self.lookup_node(name, variant)?;
    let node = &self.nodes[self.graph[idx]];
    if node.complexity.is_empty() {
        None
    } else {
        Some(node.complexity)
    }
}
```

**Step 6: Write test for variant_complexity**

```rust
#[test]
fn test_variant_complexity() {
    let graph = ReductionGraph::new();
    let variant = ReductionGraph::variant_to_map(&[("graph", "SimpleGraph"), ("weight", "i32")]);
    let complexity = graph.variant_complexity("MaximumIndependentSet", &variant);
    assert!(complexity.is_some());
    assert!(!complexity.unwrap().is_empty());
}
```

**Step 7: Build and test**

Run: `cargo test`
Expected: PASS

**Step 8: Commit**

```bash
git add src/rules/graph.rs src/unit_tests/rules/graph.rs
git commit -m "feat: ReductionGraph builds nodes from VariantEntry with complexity"
```

---

### Task 8: Update JSON export with complexity field

**Files:**
- Modify: `src/rules/graph.rs` (NodeJson struct and to_json())

**Step 1: Add complexity to NodeJson**

```rust
#[derive(Debug, Clone, Serialize)]
pub(crate) struct NodeJson {
    pub(crate) name: String,
    pub(crate) variant: BTreeMap<String, String>,
    pub(crate) category: String,
    pub(crate) doc_path: String,
    /// Worst-case time complexity expression (empty if not declared).
    pub(crate) complexity: String,
}
```

**Step 2: Update to_json() to populate complexity**

In the node-building section of `to_json()`, add:

```rust
let complexity = self.nodes[i].complexity.to_string();
// ... in NodeJson construction:
NodeJson {
    name: node.name.to_string(),
    variant: node.variant.clone(),
    category,
    doc_path,
    complexity,
}
```

**Step 3: Build and test**

Run: `cargo test`
Expected: PASS (existing tests may need updating if they assert exact JSON structure)

**Step 4: Commit**

```bash
git add src/rules/graph.rs
git commit -m "feat: include complexity in graph JSON export"
```

---

### Task 9: Update CLI `pred show` to display complexity

**Files:**
- Modify: `problemreductions-cli/src/commands/graph.rs`

**Step 1: Add complexity to variant display**

In the `show_problem_inner` function, update the variants section. For each variant, also show complexity.

Find where variants are printed (human-readable output) and add complexity:

```
Variants:
  /SimpleGraph/i32  complexity: 2^num_vertices
  /KingsSubgraph/i32  complexity: 2^num_vertices
```

**Step 2: Add complexity to JSON output**

In the JSON output path of `show_problem_inner`, include complexity per variant.

**Step 3: Update MCP show_problem_inner**

In `problemreductions-cli/src/mcp/tools.rs`, update the MCP `show_problem` output to include complexity per variant.

**Step 4: Build and test**

Run: `cargo build && cargo test`
Expected: PASS

**Step 5: Smoke test**

Run: `cargo run -p problemreductions-cli -- show MIS`
Expected: Variants section shows complexity for each variant.

**Step 6: Commit**

```bash
git add problemreductions-cli/src/commands/graph.rs problemreductions-cli/src/mcp/tools.rs
git commit -m "feat: display per-variant complexity in pred show"
```

---

### Task 10: Update graph JSON test data

**Files:**
- Modify: `tests/data/reduction_graph.json` (if it exists and is checked in tests)
- Modify: Any tests that assert exact JSON structure

**Step 1: Regenerate graph JSON**

Run: `make rust-export`

**Step 2: Update any snapshot tests**

Check for tests that compare against stored JSON. Update expected values.

**Step 3: Run full test suite**

Run: `make check`
Expected: PASS (fmt + clippy + all tests)

**Step 4: Commit**

```bash
git add -A
git commit -m "chore: update test data for variant complexity"
```

---

### Task 11: Final verification

**Step 1: Run full CI check**

Run: `make check`
Expected: PASS

**Step 2: Run CLI demo**

Run: `make cli-demo`
Expected: PASS

**Step 3: Test compile-time enforcement**

Temporarily add a bogus reduction (or comment out a declare_variants entry) and verify the build fails with a clear error about `DeclaredVariant`.

**Step 4: Verify JSON export**

Run: `cargo run -p problemreductions-cli -- show MIS --json | python3 -m json.tool | head -30`
Expected: JSON includes complexity per variant.
