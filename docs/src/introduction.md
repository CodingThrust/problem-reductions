# Problem Reductions

A Rust library for reducing NP-hard problems.

## Overview

**problemreductions** provides implementations of various NP-hard computational problems and reduction rules between them. It is designed for algorithm research, education, and quantum optimization studies.

For theoretical background and correctness proofs, see the [PDF manual](https://codingthrust.github.io/problem-reductions/reductions.pdf).

## Reduction Graph

<div id="cy" style="width: 100%; height: 500px; border: 1px solid #ddd; border-radius: 4px; background: #fafafa;"></div>
<div style="margin-top: 8px; display: flex; align-items: center; justify-content: space-between; flex-wrap: wrap; gap: 8px;">
  <div id="legend" style="font-family: sans-serif; font-size: 13px;">
    <span style="display:inline-block;width:14px;height:14px;background:#c8f0c8;border:1px solid #999;margin-right:3px;vertical-align:middle;border-radius:2px;"></span>Graph
    <span style="display:inline-block;width:14px;height:14px;background:#f0c8c8;border:1px solid #999;margin-left:10px;margin-right:3px;vertical-align:middle;border-radius:2px;"></span>Set
    <span style="display:inline-block;width:14px;height:14px;background:#f0f0a0;border:1px solid #999;margin-left:10px;margin-right:3px;vertical-align:middle;border-radius:2px;"></span>Optimization
    <span style="display:inline-block;width:14px;height:14px;background:#c8c8f0;border:1px solid #999;margin-left:10px;margin-right:3px;vertical-align:middle;border-radius:2px;"></span>Satisfiability
    <span style="display:inline-block;width:14px;height:14px;background:#f0c8e0;border:1px solid #999;margin-left:10px;margin-right:3px;vertical-align:middle;border-radius:2px;"></span>Specialized
  </div>
  <div>
    <span id="instructions" style="font-family: sans-serif; font-size: 13px; color: #666;">Click a node to start path selection</span>
    <button id="clear-btn" style="display:none; margin-left:8px; padding:3px 10px; cursor:pointer; font-size:12px;" onclick="clearPath()">Clear</button>
  </div>
</div>
<div style="margin-top: 8px; font-family: sans-serif; font-size: 12px; color: #888;">
  Click two nodes to find a reduction path. Double-click a node to view its API docs. Scroll to zoom, drag to pan.
</div>
<div id="tooltip" style="display:none; position:absolute; background:white; border:1px solid #ccc; padding:8px 12px; border-radius:4px; font-family:sans-serif; font-size:13px; box-shadow:0 2px 8px rgba(0,0,0,0.15); pointer-events:none; z-index:1000;"></div>

<script src="static/cytoscape.min.js"></script>
<script>
(function() {
  var categoryColors = {
    graph: '#c8f0c8', set: '#f0c8c8', optimization: '#f0f0a0',
    satisfiability: '#c8c8f0', specialized: '#f0c8e0'
  };
  var categoryBorders = {
    graph: '#4a8c4a', set: '#8c4a4a', optimization: '#8c8c4a',
    satisfiability: '#4a4a8c', specialized: '#8c4a6a'
  };

  fetch('reductions/reduction_graph.json')
    .then(function(r) { if (!r.ok) throw new Error('HTTP ' + r.status); return r.json(); })
    .then(function(data) {
      var baseNodes = data.nodes.filter(function(n) {
        return !n.variant || Object.keys(n.variant).length === 0;
      });
      var baseNames = {};
      baseNodes.forEach(function(n) { baseNames[n.name] = true; });

      var edgeMap = {};
      data.edges.forEach(function(e) {
        if (!baseNames[e.source.name] || !baseNames[e.target.name]) return;
        var fwd = e.source.name + '->' + e.target.name;
        var rev = e.target.name + '->' + e.source.name;
        if (edgeMap[rev]) { edgeMap[rev].bidirectional = true; }
        else if (!edgeMap[fwd]) {
          edgeMap[fwd] = { source: e.source.name, target: e.target.name, bidirectional: e.bidirectional || false, overhead: e.overhead || [] };
        }
      });

      var elements = [];
      baseNodes.forEach(function(n) {
        elements.push({ data: { id: n.name, label: n.name, category: n.category || 'other', doc_path: n.doc_path || '' } });
      });
      Object.keys(edgeMap).forEach(function(k) {
        var e = edgeMap[k];
        elements.push({ data: { id: k, source: e.source, target: e.target, bidirectional: e.bidirectional, overhead: e.overhead } });
      });

      var cy = cytoscape({
        container: document.getElementById('cy'),
        elements: elements,
        style: [
          { selector: 'node', style: {
            'label': 'data(label)', 'text-valign': 'center', 'text-halign': 'center',
            'font-size': '11px', 'font-family': 'monospace',
            'width': function(ele) { return Math.max(ele.data('label').length * 8 + 16, 60); },
            'height': 32, 'shape': 'round-rectangle',
            'background-color': function(ele) { return categoryColors[ele.data('category')] || '#f0f0f0'; },
            'border-width': 1.5,
            'border-color': function(ele) { return categoryBorders[ele.data('category')] || '#999'; },
            'text-wrap': 'none', 'padding': '6px', 'cursor': 'pointer'
          }},
          { selector: 'edge', style: {
            'width': 2, 'line-color': '#999', 'target-arrow-color': '#999', 'target-arrow-shape': 'triangle',
            'source-arrow-color': '#999',
            'source-arrow-shape': function(ele) { return ele.data('bidirectional') ? 'triangle' : 'none'; },
            'curve-style': 'bezier', 'arrow-scale': 0.8, 'cursor': 'pointer'
          }},
          { selector: '.highlighted', style: {
            'background-color': '#ff6b6b', 'border-color': '#cc0000', 'border-width': 3, 'z-index': 10
          }},
          { selector: 'edge.highlighted', style: {
            'line-color': '#ff4444', 'target-arrow-color': '#ff4444', 'source-arrow-color': '#ff4444', 'width': 4, 'z-index': 10
          }},
          { selector: '.selected-node', style: {
            'border-color': '#0066cc', 'border-width': 3, 'background-color': '#cce0ff'
          }}
        ],
        layout: {
          name: 'cose', animate: false,
          nodeRepulsion: function() { return 8000; },
          idealEdgeLength: function() { return 120; },
          gravity: 0.3, numIter: 500, padding: 30
        },
        userZoomingEnabled: true, userPanningEnabled: true, boxSelectionEnabled: false
      });

      // Tooltip
      var tooltip = document.getElementById('tooltip');
      cy.on('mouseover', 'node', function(evt) {
        var d = evt.target.data();
        tooltip.innerHTML = '<strong>' + d.label + '</strong><br>Category: ' + d.category + '<br><em>Double-click to view API docs</em>';
        tooltip.style.display = 'block';
      });
      cy.on('mousemove', 'node', function(evt) {
        var pos = evt.renderedPosition || evt.position;
        var rect = document.getElementById('cy').getBoundingClientRect();
        tooltip.style.left = (rect.left + window.scrollX + pos.x + 15) + 'px';
        tooltip.style.top = (rect.top + window.scrollY + pos.y - 10) + 'px';
      });
      cy.on('mouseout', 'node', function() { tooltip.style.display = 'none'; });

      // Edge tooltip
      cy.on('mouseover', 'edge', function(evt) {
        var d = evt.target.data();
        var arrow = d.bidirectional ? ' \u2194 ' : ' \u2192 ';
        var html = '<strong>' + d.source + arrow + d.target + '</strong>';
        if (d.overhead && d.overhead.length > 0) {
          html += '<br>' + d.overhead.map(function(o) { return '<code>' + o.field + '</code> = <code>' + o.formula + '</code>'; }).join('<br>');
        }
        html += '<br><em>Click to highlight</em>';
        tooltip.innerHTML = html;
        tooltip.style.display = 'block';
      });
      cy.on('mousemove', 'edge', function(evt) {
        var pos = evt.renderedPosition || evt.position;
        var rect = document.getElementById('cy').getBoundingClientRect();
        tooltip.style.left = (rect.left + window.scrollX + pos.x + 15) + 'px';
        tooltip.style.top = (rect.top + window.scrollY + pos.y - 10) + 'px';
      });
      cy.on('mouseout', 'edge', function() { tooltip.style.display = 'none'; });

      // Double-click to navigate to rustdoc API page
      cy.on('dbltap', 'node', function(evt) {
        var d = evt.target.data();
        if (d.doc_path) {
          window.location.href = 'api/problemreductions/' + d.doc_path;
        }
      });

      // Single-click path selection
      var selectedNode = null;
      var instructions = document.getElementById('instructions');
      var clearBtn = document.getElementById('clear-btn');

      cy.on('tap', 'node', function(evt) {
        var node = evt.target;
        if (!selectedNode) {
          selectedNode = node;
          node.addClass('selected-node');
          instructions.textContent = 'Now click a target node to find path from ' + node.data('label');
        } else if (node === selectedNode) {
          clearPath();
        } else {
          var dijkstra = cy.elements().dijkstra({ root: selectedNode, directed: true });
          var path = dijkstra.pathTo(node);
          cy.elements().removeClass('highlighted selected-node');
          if (path && path.length > 0) {
            path.addClass('highlighted');
            instructions.textContent = 'Path: ' + path.nodes().map(function(n) { return n.data('label'); }).join(' \u2192 ');
          } else {
            instructions.textContent = 'No path from ' + selectedNode.data('label') + ' to ' + node.data('label');
          }
          clearBtn.style.display = 'inline';
          selectedNode = null;
        }
      });

      cy.on('tap', 'edge', function(evt) {
        var edge = evt.target;
        var d = edge.data();
        cy.elements().removeClass('highlighted selected-node');
        edge.addClass('highlighted');
        edge.source().addClass('highlighted');
        edge.target().addClass('highlighted');
        var arrow = d.bidirectional ? ' \u2194 ' : ' \u2192 ';
        var text = d.source + arrow + d.target;
        if (d.overhead && d.overhead.length > 0) {
          text += '  |  ' + d.overhead.map(function(o) { return o.field + ' = ' + o.formula; }).join(', ');
        }
        instructions.textContent = text;
        clearBtn.style.display = 'inline';
        selectedNode = null;
      });

      cy.on('tap', function(evt) { if (evt.target === cy) { clearPath(); } });

      window.clearPath = function() {
        cy.elements().removeClass('highlighted selected-node');
        selectedNode = null;
        instructions.textContent = 'Click a node to start path selection';
        clearBtn.style.display = 'none';
      };
    })
    .catch(function(err) {
      document.getElementById('cy').innerHTML = '<p style="padding:1em;color:#c00;">Failed to load reduction graph: ' + err.message + '</p>';
    });
})();
</script>

## Problem Variants

Problems are parameterized by graph type `G` and weight type `W`. The base variant uses `SimpleGraph` and `Unweighted` (e.g., `MaximumIndependentSet`). Graph variants specify a different topology (e.g., `MaximumIndependentSet/GridGraph`), and weighted variants use numeric weights (e.g., `MaximumIndependentSet/Weighted`). Variants appear as separate nodes in the reduction graph when they have distinct reductions.

The library supports four graph topologies:

- **SimpleGraph** — standard adjacency-based graph ([petgraph](https://docs.rs/petgraph))
- **GridGraph** — vertices on a regular grid with nearest-neighbor edges
- **UnitDiskGraph** — geometric graph where edges connect vertices within a distance threshold (for quantum hardware mapping)
- **HyperGraph** — generalized edges connecting any number of vertices

## Quick Example

```rust
use problemreductions::prelude::*;

// Create an Independent Set problem on a triangle graph
let problem = MaximumIndependentSet::<i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);

// Solve with brute force
let solver = BruteForce::new();
let solutions = solver.find_best(&problem);

// Maximum independent set in a triangle has size 1
assert!(solutions.iter().all(|s| s.iter().sum::<usize>() == 1));
```

## License

MIT License
