# DRY mdBook Documentation Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Remove hardcoded problem/reduction docs from mdBook, make the interactive graph the primary navigation hub linking to rustdoc, and deploy CLAUDE.md in the book.

**Architecture:** The interactive reduction graph on the introduction page becomes the main entry point for browsing problems and reductions — nodes link to rustdoc struct pages, edges link to rustdoc reduction module pages. All category-specific problem pages and reduction detail pages are removed. Topology is explained as part of variant design in the introduction. CLAUDE.md is auto-copied into the book at build time.

**Tech Stack:** Rust (registry metadata), JavaScript (Cytoscape.js graph), mdBook, Makefile

---

### Task 1: Add `doc_path` to `NodeJson` and `EdgeJson`

**Files:**
- Modify: `src/rules/graph.rs:35-62` (NodeJson, EdgeJson structs and `to_json()`)
- Modify: `src/rules/registry.rs:37-49` (ReductionEntry struct)

**Step 1: Add `doc_module` field to `ReductionEntry`**

Add a `doc_module: &'static str` field to `ReductionEntry` in `src/rules/registry.rs`. This holds the module name for rustdoc linking (e.g., `"vertexcovering_independentset"`).

**Step 2: Update the `#[reduction]` macro to emit `doc_module`**

In the proc macro crate (`problemreductions-macros`), update the `#[reduction]` attribute to automatically set `doc_module` from the source file stem. The macro already generates `inventory::submit!` calls — add `doc_module: env!("CARGO_PKG_NAME")` or derive from the impl block's module context. Alternatively, use `module_path!()` at the submit site and strip the crate prefix at export time.

**Step 3: Add `doc_path` to `NodeJson` and `EdgeJson`**

```rust
pub struct NodeJson {
    pub name: String,
    pub variant: BTreeMap<String, String>,
    pub category: String,
    pub doc_path: String,  // e.g., "models/graph/independent_set"
}

pub struct EdgeJson {
    pub source: VariantRef,
    pub target: VariantRef,
    pub bidirectional: bool,
    pub doc_module: String,  // e.g., "vertexcovering_independentset"
}
```

**Step 4: Compute `doc_path` in `to_json()`**

In `ReductionGraph::to_json()`, compute node doc paths from name + category using CamelCase→snake_case conversion:
- `"IndependentSet"` + `"graph"` → `"models/graph/independent_set"`
- `"CircuitSAT"` + `"specialized"` → `"models/specialized/circuit_sat"`

For edges, use the `doc_module` from `ReductionEntry`.

**Step 5: Verify JSON output**

Run: `cargo run --example export_graph`

Check that `docs/paper/reduction_graph.json` contains `doc_path` on nodes and `doc_module` on edges.

**Step 6: Run tests**

Run: `make test`
Expected: All pass (JSON format change doesn't break existing tests since tests don't check JSON fields exhaustively).

**Step 7: Commit**

```bash
git add src/rules/graph.rs src/rules/registry.rs
git commit -m "feat(registry): add doc_path to reduction graph JSON nodes and edges"
```

---

### Task 2: Update interactive graph to link to rustdoc

**Files:**
- Modify: `docs/src/introduction.md` (Cytoscape.js code)

**Step 1: Update double-click handler for nodes**

Change the existing `dbltap` handler to construct rustdoc URLs from `doc_path`:

```javascript
cy.on('dbltap', 'node', function(evt) {
    var d = evt.target.data();
    if (d.doc_path) {
        window.location.href = '../' + d.doc_path + '/index.html';
    }
});
```

Note: the `../` prefix is needed because the introduction page is at `book/introduction.html` and rustdoc is at `book/api/problemreductions/...`.

**Step 2: Pass `doc_path` through to Cytoscape node data**

Update the node creation to include `doc_path` from the JSON:

```javascript
baseNodes.forEach(function(n) {
    elements.push({ data: {
        id: n.name, label: n.name,
        category: n.category || 'other',
        doc_path: n.doc_path || ''
    }});
});
```

**Step 3: Add edge click handler**

Add a tap handler for edges that opens the reduction module docs:

```javascript
cy.on('tap', 'edge', function(evt) {
    var d = evt.target.data();
    if (d.doc_module) {
        window.open('../api/problemreductions/rules/' + d.doc_module + '/index.html', '_blank');
    }
});
```

**Step 4: Pass `doc_module` through to Cytoscape edge data**

Update edge creation to include `doc_module` from the JSON.

**Step 5: Update tooltip to show link hints**

Update the node tooltip to say "Double-click to view API docs" and add edge hover tooltip showing source→target with "Click to view reduction docs".

**Step 6: Update instruction text**

Change the hint text to mention all three interactions: path finding (click two nodes), API docs (double-click node), reduction docs (click edge).

**Step 7: Test locally**

Run: `make doc` then open `docs/book/introduction.html`

Verify:
- Double-click a node → navigates to rustdoc page for that problem
- Click an edge → opens reduction module docs in new tab
- Path finding still works (click node → click another node)

**Step 8: Commit**

```bash
git add docs/src/introduction.md
git commit -m "feat(docs): link graph nodes to rustdoc, edges to reduction modules"
```

---

### Task 3: Add variant design and topology to introduction

**Files:**
- Modify: `docs/src/introduction.md`

**Step 1: Add paper download link**

After the library description, add:
```markdown
For theoretical background and correctness proofs, see the [PDF manual](https://codingthrust.github.io/problem-reductions/reductions.pdf).
```

**Step 2: Add variant design section**

After the reduction graph, add a "## Problem Variants" section explaining:
- Problems are parameterized by graph type `G` and weight type `W`
- Base variant: `IndependentSet` (SimpleGraph, unweighted)
- Graph variants: `IndependentSet/GridGraph`, `IndependentSet/UnitDiskGraph`
- Weighted variants: `IndependentSet/Weighted`
- How variants appear as separate nodes in the reduction graph

Keep this concise — 2-3 paragraphs max. Link to rustdoc for full API details.

**Step 3: Add topology overview**

Within the variant design section, briefly describe the 4 graph types:
- **SimpleGraph**: Standard adjacency-based graph (petgraph)
- **GridGraph**: Regular grid layout
- **UnitDiskGraph**: Geometric graph with distance threshold (for quantum hardware mapping)
- **HyperGraph**: Edges can connect any number of vertices

One sentence each, with links to their rustdoc pages.

**Step 4: Remove problem categories table**

The existing table duplicating problem names is no longer needed — the interactive graph serves this purpose. Remove the `## Problem Categories` section.

**Step 5: Commit**

```bash
git add docs/src/introduction.md
git commit -m "docs: add variant design, topology overview, and paper link to introduction"
```

---

### Task 4: Merge reductions content into getting-started

**Files:**
- Modify: `docs/src/getting-started.md`
- Delete: `docs/src/reductions/using.md`

**Step 1: Add reduction chaining section**

Add the "Chaining Reductions" example from `reductions/using.md` to `getting-started.md`, after the existing "Applying Reductions" section.

**Step 2: Add type safety section**

Add the "Type Safety" compile_fail example from `reductions/using.md`.

**Step 3: Update "Next Steps" links**

Replace the old links to problems/reductions with:
- Link to the interactive reduction graph on the introduction page
- Link to the API reference
- Link to solvers

**Step 4: Commit**

```bash
git add docs/src/getting-started.md
git commit -m "docs: merge reduction usage into getting-started"
```

---

### Task 5: Remove obsolete pages and update SUMMARY.md

**Files:**
- Delete: `docs/src/problems/index.md`
- Delete: `docs/src/problems/graph.md`
- Delete: `docs/src/problems/satisfiability.md`
- Delete: `docs/src/problems/optimization.md`
- Delete: `docs/src/problems/set.md`
- Delete: `docs/src/problems/specialized.md`
- Delete: `docs/src/reductions/index.md`
- Delete: `docs/src/reductions/using.md`
- Delete: `docs/src/reductions/available.md`
- Delete: `docs/src/reductions/graph.md`
- Delete: `docs/src/topology.md`
- Modify: `docs/src/SUMMARY.md`

**Step 1: Update SUMMARY.md**

```markdown
[Introduction](./introduction.md)

# User Guide

- [Getting Started](./getting-started.md)
- [Solvers](./solvers.md)
- [File I/O](./io.md)

# Developer Guide

- [API Reference](./api.md)
- [Contributing](./contributing.md)
- [CLAUDE.md](./claude.md)
```

**Step 2: Delete obsolete files**

Remove all files listed above. Keep `docs/src/reductions/reduction_graph.json` (still needed by the interactive graph).

**Step 3: Verify mdBook builds**

Run: `make doc`
Expected: No broken internal links (mdBook warns on dead links).

**Step 4: Commit**

```bash
git add -A docs/src/
git commit -m "refactor(docs): remove hardcoded problem/reduction pages"
```

---

### Task 6: Deploy CLAUDE.md in mdBook and update contributing

**Files:**
- Modify: `Makefile` (doc and mdbook targets)
- Modify: `docs/src/contributing.md`
- Create (build step): `docs/src/claude.md` (auto-copied from `.claude/CLAUDE.md`)

**Step 1: Update Makefile to copy CLAUDE.md**

In both `doc:` and `mdbook:` targets, add before `mdbook build`:
```makefile
cp .claude/CLAUDE.md docs/src/claude.md
```

**Step 2: Rewrite contributing.md**

Replace the current content with a human-oriented guide that does NOT duplicate CLAUDE.md:
- Welcome message and link to [CLAUDE.md](./claude.md) for commands and architecture
- How to find issues to work on (link to GitHub issues)
- PR workflow: branch → implement → test → PR
- Authorship recognition (from README)
- Link to the AI-assisted workflow (`/issue-to-pr` skill)
- Code style summary (fmt, clippy, doc comments — one sentence each)

**Step 3: Add CLAUDE.md to .gitignore for docs/src/**

Add `docs/src/claude.md` to `.gitignore` so the auto-copied file isn't committed.

**Step 4: Verify build**

Run: `make doc`
Expected: CLAUDE.md page renders in the book, contributing.md links to it correctly.

**Step 5: Commit**

```bash
git add Makefile docs/src/contributing.md .gitignore
git commit -m "docs: deploy CLAUDE.md in mdBook, rewrite contributing page"
```

---

### Task 7: Update README contributing section

**Files:**
- Modify: `README.md`

**Step 1: Expand contributing section**

Keep the existing authorship recognition and step-by-step workflow. Add/update:
- Link to the deployed CLAUDE.md page in the mdBook for commands and architecture
- Keep the `make help` reference
- Ensure no overlap with CLAUDE.md content (don't list individual make commands or architecture details)

**Step 2: Commit**

```bash
git add README.md
git commit -m "docs: update README contributing section with CLAUDE.md link"
```

---

### Task 8: Final verification

**Step 1: Run full checks**

```bash
make test clippy
make doc
make paper
```

Expected: All pass, no warnings.

**Step 2: Verify in browser**

Open `docs/book/index.html` and check:
- Interactive graph renders with all nodes
- Double-click node → rustdoc struct page loads
- Click edge → reduction module docs open
- Path finding works
- Variant design section is present
- Paper download link works
- Getting-started includes reduction chaining examples
- Contributing links to CLAUDE.md page
- CLAUDE.md page renders correctly
- No dead internal links

**Step 3: Commit any final fixes**

```bash
git add -A
git commit -m "fix: final verification fixes for DRY mdBook docs"
```
