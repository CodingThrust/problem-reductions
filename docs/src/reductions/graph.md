# Reduction Graph

The `ReductionGraph` allows discovering reduction paths between problem types.

## Reduction Diagram

Click two nodes to highlight the shortest reduction path between them.

<div id="controls" style="margin-bottom: 10px; font-family: sans-serif; font-size: 14px;">
  <span id="instructions" style="color: #666;">Click a node to start path selection</span>
  <button id="clear-btn" style="display:none; margin-left:10px; padding:4px 12px; cursor:pointer;" onclick="clearPath()">Clear Path</button>
</div>
<div id="cy" style="width: 100%; height: 500px; border: 1px solid #ddd; background: #fafafa;"></div>
<div id="legend" style="margin-top: 10px; font-family: sans-serif; font-size: 13px;">
  <span style="display:inline-block;width:14px;height:14px;background:#c8f0c8;border:1px solid #999;margin-right:4px;vertical-align:middle;"></span> Graph
  <span style="display:inline-block;width:14px;height:14px;background:#f0c8c8;border:1px solid #999;margin-left:12px;margin-right:4px;vertical-align:middle;"></span> Set
  <span style="display:inline-block;width:14px;height:14px;background:#f0f0a0;border:1px solid #999;margin-left:12px;margin-right:4px;vertical-align:middle;"></span> Optimization
  <span style="display:inline-block;width:14px;height:14px;background:#c8c8f0;border:1px solid #999;margin-left:12px;margin-right:4px;vertical-align:middle;"></span> Satisfiability
  <span style="display:inline-block;width:14px;height:14px;background:#f0c8e0;border:1px solid #999;margin-left:12px;margin-right:4px;vertical-align:middle;"></span> Specialized
</div>
<div id="tooltip" style="display:none; position:absolute; background:white; border:1px solid #ccc; padding:8px 12px; border-radius:4px; font-family:sans-serif; font-size:13px; box-shadow:0 2px 8px rgba(0,0,0,0.15); pointer-events:none; z-index:1000;"></div>

<script src="https://unpkg.com/cytoscape@3/dist/cytoscape.min.js"></script>
<script>
(function() {
  const categoryColors = {
    graph: '#c8f0c8',
    set: '#f0c8c8',
    optimization: '#f0f0a0',
    satisfiability: '#c8c8f0',
    specialized: '#f0c8e0'
  };

  const categoryBorders = {
    graph: '#4a8c4a',
    set: '#8c4a4a',
    optimization: '#8c8c4a',
    satisfiability: '#4a4a8c',
    specialized: '#8c4a6a'
  };

  fetch('reduction_graph.json')
    .then(r => r.json())
    .then(data => {
      // Filter to base problems only (empty variant)
      const baseNodes = data.nodes.filter(n =>
        !n.variant || Object.keys(n.variant).length === 0
      );
      const baseNames = new Set(baseNodes.map(n => n.name));

      // Deduplicate edges by base name, detect bidirectionality
      const edgeMap = {};
      data.edges.forEach(e => {
        if (!baseNames.has(e.source.name) || !baseNames.has(e.target.name)) return;
        const fwd = e.source.name + '->' + e.target.name;
        const rev = e.target.name + '->' + e.source.name;
        if (edgeMap[rev]) {
          edgeMap[rev].bidirectional = true;
        } else if (!edgeMap[fwd]) {
          edgeMap[fwd] = {
            source: e.source.name,
            target: e.target.name,
            bidirectional: e.bidirectional || false
          };
        }
      });

      const elements = [];
      baseNodes.forEach(n => {
        elements.push({
          data: {
            id: n.name,
            label: n.name,
            category: n.category || 'other'
          }
        });
      });

      Object.values(edgeMap).forEach(e => {
        elements.push({
          data: {
            id: e.source + '->' + e.target,
            source: e.source,
            target: e.target,
            bidirectional: e.bidirectional
          }
        });
      });

      const cy = cytoscape({
        container: document.getElementById('cy'),
        elements: elements,
        style: [
          {
            selector: 'node',
            style: {
              'label': 'data(label)',
              'text-valign': 'center',
              'text-halign': 'center',
              'font-size': '11px',
              'font-family': 'monospace',
              'width': function(ele) { return Math.max(ele.data('label').length * 8 + 16, 60); },
              'height': 32,
              'shape': 'round-rectangle',
              'background-color': function(ele) { return categoryColors[ele.data('category')] || '#f0f0f0'; },
              'border-width': 1.5,
              'border-color': function(ele) { return categoryBorders[ele.data('category')] || '#999'; },
              'text-wrap': 'none',
              'padding': '6px'
            }
          },
          {
            selector: 'edge',
            style: {
              'width': 2,
              'line-color': '#999',
              'target-arrow-color': '#999',
              'target-arrow-shape': 'triangle',
              'source-arrow-color': '#999',
              'source-arrow-shape': function(ele) {
                return ele.data('bidirectional') ? 'triangle' : 'none';
              },
              'curve-style': 'bezier',
              'arrow-scale': 0.8
            }
          },
          {
            selector: '.highlighted',
            style: {
              'background-color': '#ff6b6b',
              'border-color': '#cc0000',
              'border-width': 3,
              'z-index': 10
            }
          },
          {
            selector: 'edge.highlighted',
            style: {
              'line-color': '#ff4444',
              'target-arrow-color': '#ff4444',
              'source-arrow-color': '#ff4444',
              'width': 4,
              'z-index': 10
            }
          },
          {
            selector: '.selected-node',
            style: {
              'border-color': '#0066cc',
              'border-width': 3,
              'background-color': '#cce0ff'
            }
          }
        ],
        layout: {
          name: 'cose',
          animate: false,
          nodeRepulsion: function() { return 8000; },
          idealEdgeLength: function() { return 120; },
          gravity: 0.3,
          numIter: 500,
          padding: 30
        },
        userZoomingEnabled: true,
        userPanningEnabled: true,
        boxSelectionEnabled: false
      });

      // Tooltip
      const tooltip = document.getElementById('tooltip');
      cy.on('mouseover', 'node', function(evt) {
        const node = evt.target;
        const d = node.data();
        tooltip.innerHTML = '<strong>' + d.label + '</strong><br>Category: ' + d.category;
        tooltip.style.display = 'block';
      });
      cy.on('mousemove', 'node', function(evt) {
        const pos = evt.renderedPosition || evt.position;
        const container = document.getElementById('cy');
        const rect = container.getBoundingClientRect();
        tooltip.style.left = (rect.left + pos.x + 15) + 'px';
        tooltip.style.top = (rect.top + pos.y - 10) + 'px';
      });
      cy.on('mouseout', 'node', function() {
        tooltip.style.display = 'none';
      });

      // Path selection
      let selectedNode = null;
      const instructions = document.getElementById('instructions');
      const clearBtn = document.getElementById('clear-btn');

      cy.on('tap', 'node', function(evt) {
        const node = evt.target;
        if (!selectedNode) {
          selectedNode = node;
          node.addClass('selected-node');
          instructions.textContent = 'Click another node to find the path from ' + node.data('label');
        } else if (node === selectedNode) {
          // Clicking same node deselects
          clearPath();
        } else {
          // Find shortest path
          const dijkstra = cy.elements().dijkstra({
            root: selectedNode,
            directed: true
          });
          const path = dijkstra.pathTo(node);
          if (path && path.length > 0) {
            cy.elements().removeClass('highlighted selected-node');
            path.addClass('highlighted');
            instructions.textContent = 'Path: ' + path.nodes().map(n => n.data('label')).join(' → ');
            clearBtn.style.display = 'inline';
          } else {
            instructions.textContent = 'No path found between ' + selectedNode.data('label') + ' and ' + node.data('label');
            clearBtn.style.display = 'inline';
          }
          selectedNode = null;
        }
      });

      // Clear path on background click
      cy.on('tap', function(evt) {
        if (evt.target === cy) {
          clearPath();
        }
      });

      window.clearPath = function() {
        cy.elements().removeClass('highlighted selected-node');
        selectedNode = null;
        instructions.textContent = 'Click a node to start path selection';
        clearBtn.style.display = 'none';
      };
    });
})();
</script>

## Usage

```rust
use problemreductions::prelude::*;
use problemreductions::rules::ReductionGraph;

let graph = ReductionGraph::new();

// Check if direct reduction exists
let has_direct = graph.has_direct_reduction::<IndependentSet<i32>, VertexCovering<i32>>();

// Find all paths between types
let paths = graph.find_paths::<SetPacking<i32>, VertexCovering<i32>>();

// Find shortest path
let shortest = graph.find_shortest_path::<SetPacking<i32>, VertexCovering<i32>>();

// Get statistics
println!("Types: {}, Reductions: {}", graph.num_types(), graph.num_reductions());
```

## Registered Reductions

| Source | Target | Bidirectional |
|--------|--------|---------------|
| IndependentSet | VertexCovering | Yes |
| IndependentSet | SetPacking | Yes |
| VertexCovering | SetCovering | No |
| Matching | SetPacking | No |
| SpinGlass&lt;f64&gt; | QUBO&lt;f64&gt; | Yes |
| SpinGlass&lt;i32&gt; | MaxCut&lt;i32&gt; | Yes |
| Satisfiability | KSatisfiability&lt;3&gt; | Yes |
| Satisfiability | IndependentSet | No |
| Satisfiability | Coloring | No |
| Satisfiability | DominatingSet | No |
| CircuitSAT | SpinGlass&lt;i32&gt; | No |
| Factoring | CircuitSAT | No |
| Coloring | ILP | No |
| Factoring | ILP | No |

## API

```rust
impl ReductionGraph {
    /// Create a new reduction graph with all registered reductions.
    pub fn new() -> Self;

    /// Check if a direct reduction exists from S to T.
    pub fn has_direct_reduction<S: 'static, T: 'static>(&self) -> bool;

    /// Find all paths from source to target type.
    pub fn find_paths<S: 'static, T: 'static>(&self) -> Vec<ReductionPath>;

    /// Find the shortest path from source to target type.
    pub fn find_shortest_path<S: 'static, T: 'static>(&self) -> Option<ReductionPath>;

    /// Get all registered problem type names.
    pub fn problem_types(&self) -> Vec<&'static str>;

    /// Get the number of registered problem types.
    pub fn num_types(&self) -> usize;

    /// Get the number of registered reductions.
    pub fn num_reductions(&self) -> usize;
}
```
