# JSON Schema & Interactive Visualization Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace manual Rust struct definitions in typst with auto-generated JSON (issue #33), and move the reduction diagram from typst to an interactive Cytoscape.js page in mdBook (issue #34).

**Architecture:** Use `inventory` crate (already used for `ReductionEntry`) to auto-register problem schema entries. Export to JSON via an example binary. For the interactive diagram, embed Cytoscape.js in an mdBook page with path exploration.

**Tech Stack:** Rust (`inventory`, `serde_json`), Typst, Cytoscape.js (CDN), mdBook

---

### Task 1: Add `FieldInfo` and `ProblemSchemaEntry` to registry

**Files:**
- Modify: `src/registry/info.rs`
- Create: `src/registry/schema.rs`
- Modify: `src/registry/mod.rs`

**Step 1: Add `FieldInfo` struct to `src/registry/info.rs`**

Add after `ProblemInfo` impl block (after line 198):

```rust
/// Description of a struct field for JSON schema export.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldInfo {
    /// Field name as it appears in the Rust struct.
    pub name: &'static str,
    /// Type name (e.g., "Vec<W>", "UnGraph<(), ()>").
    pub type_name: &'static str,
    /// Human-readable description of what this field represents.
    pub description: &'static str,
}
```

Add `fields` to `ProblemInfo`:
```rust
pub fields: &'static [FieldInfo],
```

Update `ProblemInfo::new` to initialize `fields: &[]` and add builder:
```rust
pub const fn with_fields(mut self, fields: &'static [FieldInfo]) -> Self {
    self.fields = fields;
    self
}
```

**Step 2: Create `src/registry/schema.rs`**

```rust
//! Problem schema registration via inventory.

use super::FieldInfo;
use serde::Serialize;

/// A registered problem schema entry for static inventory registration.
pub struct ProblemSchemaEntry {
    /// Problem name (e.g., "IndependentSet").
    pub name: &'static str,
    /// Category (e.g., "graph", "optimization").
    pub category: &'static str,
    /// Human-readable description.
    pub description: &'static str,
    /// Struct fields.
    pub fields: &'static [FieldInfo],
}

inventory::collect!(ProblemSchemaEntry);

/// JSON-serializable problem schema.
#[derive(Debug, Clone, Serialize)]
pub struct ProblemSchemaJson {
    pub name: String,
    pub category: String,
    pub description: String,
    pub fields: Vec<FieldInfoJson>,
}

/// JSON-serializable field info.
#[derive(Debug, Clone, Serialize)]
pub struct FieldInfoJson {
    pub name: String,
    pub type_name: String,
    pub description: String,
}

/// Collect all registered problem schemas into JSON-serializable form.
pub fn collect_schemas() -> Vec<ProblemSchemaJson> {
    let mut schemas: Vec<ProblemSchemaJson> = inventory::iter::<ProblemSchemaEntry>
        .into_iter()
        .map(|entry| ProblemSchemaJson {
            name: entry.name.to_string(),
            category: entry.category.to_string(),
            description: entry.description.to_string(),
            fields: entry
                .fields
                .iter()
                .map(|f| FieldInfoJson {
                    name: f.name.to_string(),
                    type_name: f.type_name.to_string(),
                    description: f.description.to_string(),
                })
                .collect(),
        })
        .collect();
    schemas.sort_by(|a, b| a.name.cmp(&b.name));
    schemas
}
```

**Step 3: Update `src/registry/mod.rs`**

Add `mod schema;` and re-export `ProblemSchemaEntry`, `ProblemSchemaJson`, `collect_schemas`.

**Step 4: Run `cargo check`**

Run: `cargo check`
Expected: Compiles with no errors.

**Step 5: Commit**

```
feat(registry): add FieldInfo and ProblemSchemaEntry for auto-discovery
```

---

### Task 2: Register all problem schemas via inventory

**Files:**
- Modify: `src/models/graph/independent_set.rs`
- Modify: `src/models/graph/vertex_covering.rs`
- Modify: `src/models/graph/max_cut.rs`
- Modify: `src/models/graph/kcoloring.rs`
- Modify: `src/models/graph/dominating_set.rs`
- Modify: `src/models/graph/matching.rs`
- Modify: `src/models/graph/clique.rs`
- Modify: `src/models/graph/maximal_is.rs`
- Modify: `src/models/set/set_packing.rs`
- Modify: `src/models/set/set_covering.rs`
- Modify: `src/models/optimization/spin_glass.rs`
- Modify: `src/models/optimization/qubo.rs`
- Modify: `src/models/optimization/ilp.rs`
- Modify: `src/models/satisfiability/sat.rs`
- Modify: `src/models/satisfiability/ksat.rs`
- Modify: `src/models/specialized/circuit.rs`
- Modify: `src/models/specialized/factoring.rs`
- Modify: `src/models/specialized/bmf.rs`
- Modify: `src/models/specialized/biclique_cover.rs`
- Modify: `src/models/specialized/paintshop.rs`

**Step 1: Add `inventory::submit!` to each problem file**

Pattern for each file — add at module level (outside impl blocks):

```rust
use crate::registry::{FieldInfo, ProblemSchemaEntry};

inventory::submit! {
    ProblemSchemaEntry {
        name: "IndependentSet",
        category: "graph",
        description: "Find maximum weight independent set in a graph",
        fields: &[
            FieldInfo { name: "graph", type_name: "UnGraph<(), ()>", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "weights", type_name: "Vec<W>", description: "Vertex weights w: V -> R" },
        ],
    }
}
```

Full list of registrations (one per problem):

| Problem | Category | Fields |
|---------|----------|--------|
| IndependentSet | graph | graph: UnGraph<(), ()>, weights: Vec\<W\> |
| VertexCovering | graph | graph: UnGraph<(), ()>, weights: Vec\<W\> |
| MaxCut | graph | graph: UnGraph<(), W>, edge_weights: Vec\<W\> |
| KColoring | graph | num_colors: usize, graph: UnGraph<(), ()> |
| DominatingSet | graph | graph: UnGraph<(), ()>, weights: Vec\<W\> |
| Matching | graph | graph: UnGraph<(), W>, edge_weights: Vec\<W\> |
| Clique | graph | graph: UnGraph<(), ()>, weights: Vec\<W\> |
| MaximalIS | graph | graph: UnGraph<(), ()>, weights: Vec\<W\> |
| SetPacking | set | sets: Vec<Vec\<usize\>>, weights: Vec\<W\> |
| SetCovering | set | universe_size: usize, sets: Vec<Vec\<usize\>>, weights: Vec\<W\> |
| SpinGlass | optimization | graph: G, couplings: Vec\<W\>, fields: Vec\<W\> |
| QUBO | optimization | num_vars: usize, matrix: Vec<Vec\<W\>> |
| ILP | optimization | num_vars: usize, bounds: Vec\<VarBounds\>, constraints: Vec\<LinearConstraint\>, objective: Vec<(usize, f64)>, sense: ObjectiveSense |
| Satisfiability | satisfiability | num_vars: usize, clauses: Vec\<CNFClause\>, weights: Vec\<W\> |
| KSatisfiability | satisfiability | num_vars: usize, clauses: Vec\<CNFClause\>, weights: Vec\<W\> |
| CircuitSAT | satisfiability | circuit: Circuit, variables: Vec\<String\>, weights: Vec\<W\> |
| Factoring | specialized | m: usize, n: usize, target: u64 |
| BMF | specialized | matrix: Vec<Vec\<bool\>>, m: usize, n: usize, k: usize |
| BicliqueCover | specialized | left_size: usize, right_size: usize, edges: Vec<(usize, usize)>, k: usize |
| PaintShop | specialized | sequence_indices: Vec\<usize\>, car_labels: Vec\<String\>, is_first: Vec\<bool\>, num_cars: usize |

**Step 2: Run `cargo test`**

Run: `make test`
Expected: All tests pass.

**Step 3: Commit**

```
feat(models): register problem schemas for all 20 problem types
```

---

### Task 3: Create export_schemas example and Makefile target

**Files:**
- Create: `examples/export_schemas.rs`
- Modify: `Makefile`

**Step 1: Create `examples/export_schemas.rs`**

```rust
use problemreductions::registry::collect_schemas;
use std::path::Path;

fn main() {
    let schemas = collect_schemas();
    println!("Collected {} problem schemas", schemas.len());

    let output_path = Path::new("docs/paper/problem_schemas.json");
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create output directory");
    }

    let json = serde_json::to_string(&schemas).expect("Failed to serialize");
    std::fs::write(output_path, &json).expect("Failed to write file");
    println!("Exported to: {}", output_path.display());
}
```

**Step 2: Run the example**

Run: `cargo run --example export_schemas`
Expected: Creates `docs/paper/problem_schemas.json` with 20 entries.

**Step 3: Add Makefile target**

Add after `export-graph` section:

```makefile
# Export problem schemas to JSON
export-schemas:
	cargo run --example export_schemas
```

Update `paper` target to include `export-schemas`:

```makefile
paper: examples
	cargo run --example export_graph
	cargo run --example export_schemas
	cd docs/paper && typst compile reductions.typ reductions.pdf
```

**Step 4: Run `make paper`**

Run: `make paper`
Expected: Generates JSON and compiles PDF.

**Step 5: Commit**

```
feat: add export_schemas example and Makefile target
```

---

### Task 4: Update typst to read struct definitions from JSON

**Files:**
- Modify: `docs/paper/reductions.typ`

**Step 1: Add JSON loader and rendering function**

Add after `load-results` (line 22):

```typst
#let problem-schemas = json("problem_schemas.json")

// Render a problem's Rust struct from the JSON schema
#let render-struct(name) = {
  let schema = problem-schemas.find(s => s.name == name)
  if schema == none { return }
  let s = schema
  let fields = s.fields.map(f => "    " + f.name + ": " + f.type_name + ",").join("\n")
  raw("pub struct " + name + " {\n" + fields + "\n}", lang: "rust", block: true)
}
```

**Step 2: Replace all manual Rust code blocks**

For each definition in the typst file, replace the manual ` ```rust ... ``` ` struct block with `#render-struct("ProblemName")`.

Example — for IndependentSet (lines 112-117), replace:
```
  ```rust
  pub struct IndependentSet<W = i32> {
      graph: UnGraph<(), ()>,
      weights: Vec<W>,
  }
  ```
```
with:
```
  #render-struct("IndependentSet")
```

Apply the same pattern for all 15+ struct definitions found in the file:
IndependentSet, VertexCovering, MaxCut, Coloring (KColoring), DominatingSet, Matching, SetPacking, SetCovering, SpinGlass, QUBO, ILP (including VarBounds and LinearConstraint), Satisfiability (including CNFClause), KSatisfiability, CircuitSAT, Factoring.

**Note:** For ILP, Satisfiability, and CircuitSAT which show supporting structs (VarBounds, LinearConstraint, CNFClause) alongside the main struct, register these as separate schema entries or keep the supporting struct definitions inline.

**Step 3: Run `make paper`**

Run: `make paper`
Expected: PDF compiles with struct definitions rendered from JSON.

**Step 4: Commit**

```
refactor(paper): render struct definitions from JSON schema (#33)
```

---

### Task 5: Remove reduction diagram from typst

**Files:**
- Modify: `docs/paper/reductions.typ`
- Delete: `docs/paper/reduction-diagram.typ`

**Step 1: Remove the import**

Remove line 2:
```typst
#import "reduction-diagram.typ": reduction-graph, graph-data
```

**Step 2: Remove `graph-data` references**

Line 90 references `#graph-data.edges.len()` and `#graph-data.nodes.len()`. Rewrite this sentence to not reference the graph data, or load the JSON directly for just the counts:
```typst
#let graph-data = json("reduction_graph.json")
```
Keep this minimal load only if the edge/node counts are needed in the text. Otherwise remove the sentence referencing the figure.

**Step 3: Remove the figure**

Remove lines 96-99:
```typst
#figure(
  reduction-graph(width: 18mm, height: 14mm),
  caption: [Reduction graph. Colors: ...]
) <fig:reduction-graph>
```

Remove the `@fig:reduction-graph` reference from line 90.

**Step 4: Delete `reduction-diagram.typ`**

Delete `docs/paper/reduction-diagram.typ`.

**Step 5: Run `make paper`**

Run: `make paper`
Expected: PDF compiles without the diagram.

**Step 6: Commit**

```
refactor(paper): remove reduction diagram from typst (#34)
```

---

### Task 6: Create interactive Cytoscape.js page in mdBook

**Files:**
- Modify: `docs/src/SUMMARY.md`
- Modify: `docs/src/reductions/graph.md` (replace static mermaid diagram)
- Modify: `Makefile` (add export-graph to doc target)

**Step 1: Update Makefile doc target**

```makefile
doc:
	cargo run --example export_graph
	cp docs/paper/reduction_graph.json docs/src/reductions/
	mdbook build docs
```

**Step 2: Replace mermaid diagram in `docs/src/reductions/graph.md`**

Replace the mermaid code block and legend (lines 7-96) with embedded Cytoscape.js HTML. Keep the Usage, Registered Reductions, and API sections.

The embedded HTML should include:
- Controls bar: instructions text + "Clear Path" button
- Canvas div for Cytoscape
- Cytoscape.js loaded from CDN (`https://unpkg.com/cytoscape@3/dist/cytoscape.min.js`)
- Inline `<script>` that:
  1. Fetches `reduction_graph.json` (relative path)
  2. Filters to base problem nodes (empty variant)
  3. Deduplicates edges by base name, detects bidirectionality
  4. Creates Cytoscape instance with cose layout
  5. Styles nodes by category color
  6. Adds hover tooltips
  7. Implements two-click path highlighting using `cy.elements().dijkstra()`
  8. Handles directed vs bidirectional edge arrows

Category colors (matching the original typst diagram):
- graph: `#c8f0c8`
- set: `#f0c8c8`
- optimization: `#f0f0a0`
- satisfiability: `#c8c8f0`
- specialized: `#f0c8e0`

**Step 3: Test locally**

Run: `mdbook serve docs --open`
Expected: Interactive diagram renders with pan/zoom, colored nodes, hover tooltips, click-to-highlight paths.

**Step 4: Commit**

```
feat(docs): interactive reduction diagram with Cytoscape.js (#34)
```

---

### Task 7: Final verification

**Step 1: Run full check**

Run: `make test clippy`
Expected: All pass.

**Step 2: Build paper**

Run: `make paper`
Expected: PDF compiles. Struct definitions rendered from JSON. No reduction diagram.

**Step 3: Build docs**

Run: `make doc`
Expected: mdBook builds. Interactive diagram works.

**Step 4: Update CLAUDE.md if needed**

Add `export-schemas` to the commands section if appropriate.

**Step 5: Commit any final cleanup**
