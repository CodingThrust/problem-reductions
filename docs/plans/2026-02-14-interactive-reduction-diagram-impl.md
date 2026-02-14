# Interactive Reduction Diagram Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Upgrade the existing Cytoscape.js reduction diagram in `docs/src/introduction.md` to support collapsible compound nodes with variant dots, ELK stress layout, edge filtering, and a search bar — while preserving existing path finding, doc links, and tooltip features.

**Architecture:** Refactor the ~340-line inline `<script>` in `introduction.md` into an external JS file (`docs/src/static/reduction-graph.js`) and CSS file (`docs/src/static/reduction-graph.css`). Add ELK.js via CDN for stress layout. Replace the flat variant positioning with Cytoscape.js compound nodes that expand/collapse on click.

**Tech Stack:** Cytoscape.js (already vendored), ELK.js + cytoscape-elk (CDN), vanilla JS

---

### Task 1: Extract JS/CSS to External Files

Extract the inline script from `introduction.md` into separate files so we can iterate on the diagram code without editing the markdown. No behavior change yet.

**Files:**
- Create: `docs/src/static/reduction-graph.js`
- Create: `docs/src/static/reduction-graph.css`
- Modify: `docs/src/introduction.md:7-371` (replace inline HTML/CSS/JS with container + script tags)
- Modify: `book.toml:13` (add to `additional-js` and `additional-css`)

**Step 1: Create the CSS file**

Extract the inline styles from the tooltip, legend, and instructions into `docs/src/static/reduction-graph.css`:

```css
#cy {
  width: 100%;
  height: 600px;
  border: 1px solid var(--sidebar-bg);
  border-radius: 4px;
  background: var(--bg);
}

#cy-controls {
  margin-top: 8px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-wrap: wrap;
  gap: 8px;
  font-family: sans-serif;
  font-size: 13px;
  color: var(--fg);
}

#legend span.swatch {
  display: inline-block;
  width: 14px;
  height: 14px;
  border: 1px solid #999;
  margin-right: 3px;
  vertical-align: middle;
  border-radius: 2px;
}

#legend span.swatch + span.swatch {
  margin-left: 10px;
}

#cy-tooltip {
  display: none;
  position: absolute;
  background: var(--bg);
  color: var(--fg);
  border: 1px solid var(--sidebar-bg);
  padding: 8px 12px;
  border-radius: 4px;
  font-family: sans-serif;
  font-size: 13px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.15);
  pointer-events: none;
  z-index: 1000;
}

#cy-help {
  margin-top: 8px;
  font-family: sans-serif;
  font-size: 12px;
  color: var(--fg);
  opacity: 0.6;
}

#clear-btn {
  display: none;
  margin-left: 8px;
  padding: 3px 10px;
  cursor: pointer;
  font-size: 12px;
}
```

**Step 2: Create the JS file**

Move the entire `(function() { ... })()` block from `introduction.md` into `docs/src/static/reduction-graph.js`. Wrap in a `DOMContentLoaded` listener so it runs after the page loads:

```javascript
document.addEventListener('DOMContentLoaded', function() {
  // Check if the cy container exists on this page
  var cyContainer = document.getElementById('cy');
  if (!cyContainer) return;

  // ... paste entire existing IIFE body here, changing:
  //   document.getElementById('tooltip') → document.getElementById('cy-tooltip')
  //   onclick="clearPath()" → handled in JS
});
```

**Step 3: Simplify the HTML in introduction.md**

Replace lines 7–371 of `introduction.md` with clean HTML that references the external files:

```html
<div id="cy"></div>
<div id="cy-controls">
  <div id="legend">
    <span class="swatch" style="background:#c8f0c8;"></span>Graph
    <span class="swatch" style="background:#f0c8c8;"></span>Set
    <span class="swatch" style="background:#f0f0a0;"></span>Optimization
    <span class="swatch" style="background:#c8c8f0;"></span>Satisfiability
    <span class="swatch" style="background:#f0c8e0;"></span>Specialized
  </div>
  <div>
    <span id="instructions">Click a node to start path selection</span>
    <button id="clear-btn">Clear</button>
  </div>
</div>
<div id="cy-help">
  Click two variant nodes to find a reduction path. Double-click a node for API docs, double-click an edge for source code. Scroll to zoom, drag to pan.
</div>
<div id="cy-tooltip"></div>
```

**Step 4: Update book.toml**

Change `additional-js` and `additional-css`:

```toml
additional-css = ["docs/src/static/theme-images.css", "docs/src/static/reduction-graph.css"]
additional-js = ["docs/src/static/cytoscape.min.js", "docs/src/static/reduction-graph.js"]
```

**Step 5: Verify**

Run: `mdbook serve` (or `make mdbook`)
Expected: The diagram looks and behaves identically to the current inline version. Path finding, tooltips, double-click doc links all work.

**Step 6: Commit**

```bash
git add docs/src/static/reduction-graph.js docs/src/static/reduction-graph.css docs/src/introduction.md book.toml
git commit -m "refactor: extract reduction diagram JS/CSS to external files"
```

---

### Task 2: Add ELK.js and Switch to Stress Layout

Replace the two-pass cose+preset layout with ELK stress layout via CDN.

**Files:**
- Modify: `docs/src/static/reduction-graph.js` (replace layout code)
- Modify: `docs/src/introduction.md` (add CDN script tags for elk)

**Step 1: Add ELK.js CDN scripts**

In `introduction.md`, before the existing `<div id="cy">`, add:

```html
<script src="https://unpkg.com/elkjs@0.9.3/lib/elk.bundled.js"></script>
<script src="https://unpkg.com/cytoscape-elk@2.2.0/cytoscape-elk.js"></script>
```

Note: `cytoscape-elk` auto-registers with Cytoscape when loaded via `<script>` tag (it detects the global `cytoscape` from the vendored file).

**Step 2: Replace the layout**

In `reduction-graph.js`, remove the entire two-pass layout approach (the `tempCy` creation in Step 1, the `positions` map, and the preset layout in Step 2). Replace with a single Cytoscape instance using ELK stress:

```javascript
var cy = cytoscape({
  container: document.getElementById('cy'),
  elements: elements,
  style: [ /* ... existing styles ... */ ],
  layout: {
    name: 'elk',
    elk: {
      algorithm: 'stress',
      'stress.desiredEdgeLength': 200,
      'nodeNode.spacing': 40,
    },
    animate: true,
    animationDuration: 500,
    padding: 40
  }
});
```

The element-building code stays the same for now (flat variant nodes) — we just change how they're positioned.

**Step 3: Verify**

Run: `make mdbook`
Expected: Diagram renders with stress layout. Nodes are positioned by graph-theoretic distance. All interactions (path finding, tooltips, doc links) still work. ELK loads from CDN (requires internet).

**Step 4: Commit**

```bash
git add docs/src/static/reduction-graph.js docs/src/introduction.md
git commit -m "feat: switch reduction diagram to ELK stress layout"
```

---

### Task 3: Implement Compound Nodes (Collapsed View)

Convert the flat variant nodes into compound parent/child structure. Start with all nodes collapsed (name-level view only).

**Files:**
- Modify: `docs/src/static/reduction-graph.js` (rewrite element building)
- Modify: `docs/src/static/reduction-graph.css` (compound node styles)

**Step 1: Rewrite element building for compound nodes**

Replace the current element-building loop with compound node logic:

```javascript
// Group nodes by name
var problems = {};
data.nodes.forEach(function(n, idx) {
  if (!problems[n.name]) {
    problems[n.name] = { category: n.category, doc_path: n.doc_path, variants: [] };
  }
  problems[n.name].variants.push({ index: idx, variant: n.variant, category: n.category, doc_path: n.doc_path });
});

var elements = [];
var parentIds = {};  // name → parent node id

Object.keys(problems).forEach(function(name) {
  var info = problems[name];
  var hasMultipleVariants = info.variants.length > 1;

  if (hasMultipleVariants) {
    // Create compound parent node
    var parentId = 'parent_' + name;
    parentIds[name] = parentId;
    elements.push({
      data: {
        id: parentId,
        label: name,
        category: info.category,
        doc_path: info.doc_path,
        isParent: true,
        variantCount: info.variants.length
      }
    });

    // Create child nodes (hidden initially — collapsed)
    info.variants.forEach(function(v) {
      var vid = variantId(name, v.variant);
      elements.push({
        data: {
          id: vid,
          parent: parentId,
          label: variantLabel(v.variant),
          fullLabel: name + ' (' + fullVariantLabel(v.variant) + ')',
          category: v.category,
          doc_path: v.doc_path,
          isVariant: true,
          problemName: name
        }
      });
    });
  } else {
    // Single variant — simple node (no parent)
    var v = info.variants[0];
    var vid = variantId(name, v.variant);
    elements.push({
      data: {
        id: vid,
        label: name,
        fullLabel: name + ' (' + fullVariantLabel(v.variant) + ')',
        category: v.category,
        doc_path: v.doc_path,
        isVariant: false,
        problemName: name
      }
    });
  }
});
```

**Step 2: Build collapsed-mode edges (name-level)**

Add merged edges between parent/simple nodes:

```javascript
// Build name-level edge map (collapse variant edges)
var nameLevelEdges = {};
data.edges.forEach(function(e) {
  var srcName = data.nodes[e.source].name;
  var dstName = data.nodes[e.target].name;
  if (srcName === dstName) return; // skip intra-problem natural casts
  var key = srcName + '->' + dstName;
  if (!nameLevelEdges[key]) {
    nameLevelEdges[key] = { count: 0, overhead: e.overhead, doc_path: e.doc_path };
  }
  nameLevelEdges[key].count++;
});

// Add collapsed edges to elements
Object.keys(nameLevelEdges).forEach(function(key) {
  var parts = key.split('->');
  var srcId = parentIds[parts[0]] || variantId(parts[0], problems[parts[0]].variants[0].variant);
  var dstId = parentIds[parts[1]] || variantId(parts[1], problems[parts[1]].variants[0].variant);
  var info = nameLevelEdges[key];
  elements.push({
    data: {
      id: 'collapsed_' + key,
      source: srcId,
      target: dstId,
      label: info.count > 1 ? '×' + info.count : '',
      edgeLevel: 'collapsed',
      overhead: info.overhead,
      doc_path: info.doc_path
    }
  });
});
```

**Step 3: Also build variant-level edges (hidden initially)**

```javascript
// Build variant-level edges (hidden, shown when expanded)
Object.keys(edgeMap).forEach(function(k) {
  var e = edgeMap[k];
  elements.push({
    data: {
      id: 'variant_' + k,
      source: e.source,
      target: e.target,
      bidirectional: e.bidirectional,
      edgeLevel: 'variant',
      overhead: e.overhead,
      doc_path: e.doc_path
    }
  });
});
```

**Step 4: Add styles for compound and collapsed nodes**

In `reduction-graph.css`, add Cytoscape compound node styling. In the JS style array:

```javascript
// Parent (compound) node — collapsed appearance
{ selector: 'node[?isParent]', style: {
  'label': 'data(label)',
  'text-valign': 'top',
  'text-halign': 'center',
  'font-size': '11px',
  'font-family': 'monospace',
  'padding': '10px',
  'background-color': function(ele) { return categoryColors[ele.data('category')] || '#f0f0f0'; },
  'border-width': 1.5,
  'border-color': function(ele) { return categoryBorders[ele.data('category')] || '#999'; },
  'shape': 'round-rectangle',
  'cursor': 'pointer'
}},
// Child variant nodes
{ selector: 'node[?isVariant]', style: {
  'label': 'data(label)',
  'text-valign': 'center',
  'text-halign': 'center',
  'font-size': '9px',
  'font-family': 'monospace',
  'width': function(ele) { return Math.max(ele.data('label').length * 5.5 + 8, 40); },
  'height': 18,
  'shape': 'round-rectangle',
  'background-color': function(ele) { return categoryColors[ele.data('category')] || '#f0f0f0'; },
  'border-width': 1,
  'border-color': function(ele) { return categoryBorders[ele.data('category')] || '#999'; },
  'cursor': 'pointer'
}},
// Hidden variant-level edges
{ selector: 'edge[edgeLevel="variant"]', style: { 'display': 'none' } },
```

**Step 5: Hide child nodes initially (collapsed state)**

After creating the Cytoscape instance:

```javascript
// Start collapsed — hide all child variant nodes
cy.nodes('[?isVariant]').style('display', 'none');
```

**Step 6: Verify**

Run: `make mdbook`
Expected: Diagram shows ~20 compound parent nodes (collapsed) with name-level edges between them. No variant dots visible yet. Multi-reduction edges show "×N" labels.

**Step 7: Commit**

```bash
git add docs/src/static/reduction-graph.js docs/src/static/reduction-graph.css
git commit -m "feat: implement compound nodes for reduction diagram (collapsed view)"
```

---

### Task 4: Implement Expand/Collapse Interaction

Add click-to-expand on parent nodes to reveal variant dots, and re-layout.

**Files:**
- Modify: `docs/src/static/reduction-graph.js` (add expand/collapse handlers + edge switching)

**Step 1: Track expanded state**

```javascript
var expandedParents = {};  // parentId → true/false
```

**Step 2: Implement expand/collapse function**

```javascript
function toggleExpand(parentNode) {
  var parentId = parentNode.id();
  var isExpanded = expandedParents[parentId];
  var children = parentNode.children();
  var name = parentNode.data('label');

  if (isExpanded) {
    // Collapse: hide children, show collapsed edges, hide variant edges
    children.style('display', 'none');
    cy.edges('[edgeLevel="collapsed"]').forEach(function(e) {
      var srcName = e.source().data('label') || e.source().data('problemName');
      var dstName = e.target().data('label') || e.target().data('problemName');
      if (srcName === name || dstName === name) {
        e.style('display', 'element');
      }
    });
    cy.edges('[edgeLevel="variant"]').forEach(function(e) {
      var srcParent = e.source().data('parent');
      var dstParent = e.target().data('parent');
      if (srcParent === parentId || dstParent === parentId) {
        e.style('display', 'none');
      }
    });
    expandedParents[parentId] = false;
  } else {
    // Expand: show children, hide collapsed edges, show variant edges
    children.style('display', 'element');
    cy.edges('[edgeLevel="collapsed"]').forEach(function(e) {
      // Hide collapsed edges connected to this parent
      if (e.source().id() === parentId || e.target().id() === parentId) {
        e.style('display', 'none');
      }
    });
    cy.edges('[edgeLevel="variant"]').forEach(function(e) {
      var srcParent = e.source().data('parent');
      var dstParent = e.target().data('parent');
      if (srcParent === parentId || dstParent === parentId) {
        e.style('display', 'element');
      }
    });
    expandedParents[parentId] = true;
  }

  // Re-layout with animation
  cy.layout({
    name: 'elk',
    elk: { algorithm: 'stress', 'stress.desiredEdgeLength': 200 },
    animate: true,
    animationDuration: 300,
    padding: 40
  }).run();
}
```

**Step 3: Wire click handler**

Replace the existing click handler with expand/collapse awareness:

```javascript
cy.on('tap', 'node[?isParent]', function(evt) {
  toggleExpand(evt.target);
});
```

Keep the existing path-selection click handler for non-parent nodes (variant nodes and simple nodes).

**Step 4: Verify**

Run: `make mdbook`
Expected:
- Click a parent node → it expands, showing variant pills inside. Collapsed edges disappear; variant-level edges appear.
- Click the expanded parent again → collapses back. Variant edges disappear; collapsed edges reappear.
- Path finding still works (on variant nodes when expanded, on parent nodes when collapsed).

**Step 5: Commit**

```bash
git add docs/src/static/reduction-graph.js
git commit -m "feat: add expand/collapse for compound variant nodes"
```

---

### Task 5: Implement Variant Edge Filtering

When a user clicks a variant dot (child node inside an expanded parent), only that variant's edges should be highlighted; all other edges fade.

**Files:**
- Modify: `docs/src/static/reduction-graph.js` (add variant filter handler)
- Modify: `docs/src/static/reduction-graph.css` (add faded styles)

**Step 1: Add CSS class for faded elements**

In the Cytoscape style array in JS:

```javascript
{ selector: '.faded', style: { 'opacity': 0.1 } },
{ selector: '.variant-selected', style: {
  'border-color': '#0066cc',
  'border-width': 2.5,
  'background-color': '#cce0ff'
}}
```

**Step 2: Add click handler for variant nodes**

```javascript
var activeVariantFilter = null;

cy.on('tap', 'node[?isVariant]', function(evt) {
  var node = evt.target;

  if (activeVariantFilter === node.id()) {
    // Toggle off — clear filter
    cy.elements().removeClass('faded variant-selected');
    activeVariantFilter = null;
    instructions.textContent = 'Click a node to start path selection';
    return;
  }

  // Apply filter
  activeVariantFilter = node.id();
  cy.elements().addClass('faded');
  node.removeClass('faded').addClass('variant-selected');

  // Show only edges connected to this variant
  var connectedEdges = node.connectedEdges('[edgeLevel="variant"]');
  connectedEdges.removeClass('faded');
  connectedEdges.connectedNodes().removeClass('faded');

  // Also un-fade the parent
  if (node.data('parent')) {
    cy.getElementById(node.data('parent')).removeClass('faded');
  }

  instructions.textContent = 'Showing edges for ' + node.data('fullLabel') + ' — click again to clear';
});
```

**Step 3: Clear filter on background click**

Update the existing background click handler:

```javascript
cy.on('tap', function(evt) {
  if (evt.target === cy) {
    clearPath();
    cy.elements().removeClass('faded variant-selected');
    activeVariantFilter = null;
  }
});
```

**Step 4: Verify**

Run: `make mdbook`
Expected:
- Expand a parent node (e.g., MaximumIndependentSet).
- Click a variant dot (e.g., "SimpleGraph, i32").
- Only edges from/to that variant are visible. All other elements fade to 10% opacity.
- Click the same variant dot again → filter clears.
- Click background → everything resets.

**Step 5: Commit**

```bash
git add docs/src/static/reduction-graph.js docs/src/static/reduction-graph.css
git commit -m "feat: add variant edge filtering on click"
```

---

### Task 6: Add Search Bar

Add a search input that filters nodes by name — matching nodes stay visible, non-matching fade.

**Files:**
- Modify: `docs/src/introduction.md` (add search input HTML)
- Modify: `docs/src/static/reduction-graph.js` (add search handler)
- Modify: `docs/src/static/reduction-graph.css` (search bar styles)

**Step 1: Add search HTML**

In `introduction.md`, add a search bar above the `#cy` div:

```html
<div id="cy-search" style="margin-bottom: 8px;">
  <input id="search-input" type="text" placeholder="Search problems..." style="
    padding: 4px 10px;
    font-size: 13px;
    font-family: sans-serif;
    border: 1px solid var(--sidebar-bg);
    border-radius: 4px;
    background: var(--bg);
    color: var(--fg);
    width: 200px;
  ">
</div>
```

**Step 2: Add search handler in JS**

```javascript
var searchInput = document.getElementById('search-input');
if (searchInput) {
  searchInput.addEventListener('input', function() {
    var query = this.value.trim().toLowerCase();
    if (query === '') {
      cy.elements().removeClass('faded');
      return;
    }
    cy.nodes().forEach(function(node) {
      var label = (node.data('label') || '').toLowerCase();
      var fullLabel = (node.data('fullLabel') || '').toLowerCase();
      if (label.includes(query) || fullLabel.includes(query)) {
        node.removeClass('faded');
      } else {
        node.addClass('faded');
      }
    });
    cy.edges().addClass('faded');
    // Un-fade edges between matched nodes
    cy.nodes().not('.faded').connectedEdges().forEach(function(edge) {
      if (!edge.source().hasClass('faded') && !edge.target().hasClass('faded')) {
        edge.removeClass('faded');
      }
    });
  });
}
```

**Step 3: Verify**

Run: `make mdbook`
Expected:
- Type "MIS" → only MaximumIndependentSet nodes visible, others fade.
- Type "QUBO" → only QUBO nodes visible.
- Clear the input → everything resets.
- Search combines with expand/collapse (expanded children also match).

**Step 4: Commit**

```bash
git add docs/src/introduction.md docs/src/static/reduction-graph.js docs/src/static/reduction-graph.css
git commit -m "feat: add search bar to reduction diagram"
```

---

### Task 7: Integrate Path Finding with Compound Nodes

Update the existing path-finding logic to work correctly with the compound node structure. Path finding should work at the variant level when nodes are expanded and at the name level when collapsed.

**Files:**
- Modify: `docs/src/static/reduction-graph.js` (update path selection handlers)

**Step 1: Update the tap handler for path selection**

The path selection needs to handle three node types: parent nodes (compound), variant nodes (children), and simple nodes (no variants). Modify the tap handler:

```javascript
cy.on('tap', 'node', function(evt) {
  var node = evt.target;

  // If tapping a parent node, expand/collapse instead of path selection
  if (node.data('isParent')) {
    toggleExpand(node);
    return;
  }

  // Variant or simple node — proceed with path selection
  if (!selectedNode) {
    selectedNode = node;
    node.addClass('selected-node');
    instructions.textContent = 'Now click a target node to find path from ' + node.data('label');
  } else if (node === selectedNode) {
    clearPath();
  } else {
    // Run Dijkstra on visible edges only
    var visibleElements = cy.elements().filter(function(ele) {
      return ele.style('display') !== 'none';
    });
    var dijkstra = visibleElements.dijkstra({ root: selectedNode, directed: true });
    var path = dijkstra.pathTo(node);
    cy.elements().removeClass('highlighted selected-node');
    if (path && path.length > 0) {
      path.addClass('highlighted');
      instructions.textContent = 'Path: ' + path.nodes().map(function(n) { return n.data('label'); }).join(' → ');
    } else {
      instructions.textContent = 'No path from ' + selectedNode.data('label') + ' to ' + node.data('label');
    }
    clearBtn.style.display = 'inline';
    selectedNode = null;
  }
});
```

**Step 2: Verify**

Run: `make mdbook`
Expected:
- Click a variant node → starts path selection. Click another → finds and highlights path.
- Click a parent node → expands/collapses (does NOT start path selection).
- Path finding uses visible edges (collapsed or variant level depending on expand state).

**Step 3: Commit**

```bash
git add docs/src/static/reduction-graph.js
git commit -m "feat: integrate path finding with compound node structure"
```

---

### Task 8: Add Natural Cast Edge Styling

Distinguish natural cast edges (same-problem, different variant) from reduction edges visually.

**Files:**
- Modify: `docs/src/static/reduction-graph.js` (tag natural cast edges, add dashed style)

**Step 1: Tag natural cast edges during element building**

When building variant-level edges, detect intra-problem edges:

```javascript
data.edges.forEach(function(e) {
  var src = data.nodes[e.source];
  var dst = data.nodes[e.target];
  var isNaturalCast = src.name === dst.name;
  // ... existing edge building, add:
  edgeData.isNaturalCast = isNaturalCast;
});
```

**Step 2: Add dashed style for natural casts**

```javascript
{ selector: 'edge[?isNaturalCast]', style: {
  'line-style': 'dashed',
  'line-color': '#bbb',
  'target-arrow-color': '#bbb',
  'width': 1
}}
```

**Step 3: Verify**

Run: `make mdbook`
Expected: When expanding MaximumIndependentSet, edges between Triangular→SimpleGraph, GridGraph→SimpleGraph etc. appear as dashed gray lines. Cross-problem reduction edges remain solid.

**Step 4: Commit**

```bash
git add docs/src/static/reduction-graph.js
git commit -m "feat: add dashed style for natural cast edges"
```

---

### Task 9: Update Help Text and Legend

Update the instructions and legend to reflect the new compound node interactions.

**Files:**
- Modify: `docs/src/introduction.md` (update legend and help text)

**Step 1: Update the help text**

```html
<div id="cy-help">
  Click a problem node to expand/collapse its variants.
  Click a variant to filter its edges.
  Click two variants to find a reduction path.
  Double-click for API docs (nodes) or source code (edges).
  Scroll to zoom, drag to pan.
</div>
```

**Step 2: Add natural cast to legend**

Add after the existing swatches:

```html
<span style="display:inline-block;width:20px;height:0;border-top:2px dashed #bbb;margin-left:10px;margin-right:3px;vertical-align:middle;"></span>Natural Cast
```

**Step 3: Verify**

Run: `make mdbook`
Expected: Legend shows all categories plus "Natural Cast" with dashed line. Help text explains the new interactions.

**Step 4: Commit**

```bash
git add docs/src/introduction.md
git commit -m "docs: update diagram legend and help text for compound nodes"
```

---

### Task 10: Final Polish and Testing

End-to-end testing of all interactions and edge cases.

**Files:**
- Possibly: `docs/src/static/reduction-graph.js` (bug fixes)

**Step 1: Test matrix**

Run `make mdbook` and verify each scenario:

| Scenario | Expected |
|----------|----------|
| Page load | ~20 collapsed nodes, name-level edges, stress layout |
| Click parent node | Expands to show variant dots, edges split |
| Click expanded parent | Collapses back, edges merge |
| Click variant dot | Filters edges, others fade to 10% |
| Click variant dot again | Clears filter |
| Click background | Resets all filters, collapses all |
| Double-click node | Opens API docs page |
| Double-click edge | Opens GitHub source in new tab |
| Path: click variant A, click variant B | Dijkstra path highlighted |
| Path: click simple node, click variant | Path works across types |
| Search: type "SAT" | Only SAT-related nodes visible |
| Search: clear input | All nodes visible |
| Zoom/pan | Works smoothly |
| ELK CDN unavailable | Graceful fallback (show error or use cose) |

**Step 2: Fix any issues found**

Address bugs from the test matrix.

**Step 3: Final commit**

```bash
git add docs/src/static/reduction-graph.js docs/src/static/reduction-graph.css docs/src/introduction.md
git commit -m "fix: polish reduction diagram interactions and edge cases"
```

---

## Summary

| Task | Description | Estimated Complexity |
|------|-------------|---------------------|
| 1 | Extract JS/CSS to external files | Low — mechanical refactor |
| 2 | Add ELK.js, switch to stress layout | Low — layout config change |
| 3 | Implement compound nodes (collapsed) | Medium — rewrite element building |
| 4 | Implement expand/collapse | Medium — state management + edge switching |
| 5 | Variant edge filtering | Low — click handler + CSS |
| 6 | Search bar | Low — input handler + filtering |
| 7 | Path finding integration | Low — handler update |
| 8 | Natural cast edge styling | Low — tag edges + CSS |
| 9 | Update help text and legend | Low — HTML changes |
| 10 | Final polish and testing | Low — E2E verification |
