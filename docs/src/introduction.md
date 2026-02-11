# Problem Reductions

A Rust library for reducing NP-hard problems.

## Overview

**problemreductions** provides implementations of various computational hard problems and reduction rules between them. It is designed for algorithm research, education, and quantum optimization studies.

For theoretical background and correctness proofs, see the [PDF manual](https://codingthrust.github.io/problem-reductions/reductions.pdf).

## Reduction Graph

<div id="cy" style="width: 100%; height: 600px; border: 1px solid var(--sidebar-bg); border-radius: 4px; background: var(--bg);"></div>
<div style="margin-top: 8px; display: flex; align-items: center; justify-content: space-between; flex-wrap: wrap; gap: 8px;">
  <div id="legend" style="font-family: sans-serif; font-size: 13px; color: var(--fg);">
    <span style="display:inline-block;width:14px;height:14px;background:#c8f0c8;border:1px solid #999;margin-right:3px;vertical-align:middle;border-radius:2px;"></span>Graph
    <span style="display:inline-block;width:14px;height:14px;background:#f0c8c8;border:1px solid #999;margin-left:10px;margin-right:3px;vertical-align:middle;border-radius:2px;"></span>Set
    <span style="display:inline-block;width:14px;height:14px;background:#f0f0a0;border:1px solid #999;margin-left:10px;margin-right:3px;vertical-align:middle;border-radius:2px;"></span>Optimization
    <span style="display:inline-block;width:14px;height:14px;background:#c8c8f0;border:1px solid #999;margin-left:10px;margin-right:3px;vertical-align:middle;border-radius:2px;"></span>Satisfiability
    <span style="display:inline-block;width:14px;height:14px;background:#f0c8e0;border:1px solid #999;margin-left:10px;margin-right:3px;vertical-align:middle;border-radius:2px;"></span>Specialized
  </div>
  <div>
    <span id="instructions" style="font-family: sans-serif; font-size: 13px; color: var(--fg);">Click a node to start path selection</span>
    <button id="clear-btn" style="display:none; margin-left:8px; padding:3px 10px; cursor:pointer; font-size:12px;" onclick="clearPath()">Clear</button>
  </div>
</div>
<div style="margin-top: 8px; font-family: sans-serif; font-size: 12px; color: var(--fg); opacity: 0.6;">
  Click two variant nodes to find a reduction path. Double-click a node for API docs, double-click an edge for source code. Scroll to zoom, drag to pan.
</div>
<div id="tooltip" style="display:none; position:absolute; background:var(--bg); color:var(--fg); border:1px solid var(--sidebar-bg); padding:8px 12px; border-radius:4px; font-family:sans-serif; font-size:13px; box-shadow:0 2px 8px rgba(0,0,0,0.15); pointer-events:none; z-index:1000;"></div>

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

  function variantId(name, variant) {
    var keys = Object.keys(variant).sort();
    return name + '/' + keys.map(function(k) { return k + '=' + variant[k]; }).join(',');
  }

  function variantLabel(variant) {
    var graph = variant.graph || 'SimpleGraph';
    var weight = variant.weight || 'Unweighted';
    var extra = Object.keys(variant).filter(function(k) { return k !== 'graph' && k !== 'weight'; });
    var parts = [];
    if (graph !== 'SimpleGraph') parts.push(graph);
    if (weight !== 'Unweighted') parts.push('Weighted');
    extra.forEach(function(k) { parts.push(k + '=' + variant[k]); });
    return parts.length > 0 ? parts.join(', ') : 'base';
  }

  function isBaseVariant(variant) {
    var graph = variant.graph || 'SimpleGraph';
    var weight = variant.weight || 'Unweighted';
    var extra = Object.keys(variant).filter(function(k) { return k !== 'graph' && k !== 'weight'; });
    return graph === 'SimpleGraph' && weight === 'Unweighted' && extra.length === 0;
  }

  fetch('reductions/reduction_graph.json')
    .then(function(r) { if (!r.ok) throw new Error('HTTP ' + r.status); return r.json(); })
    .then(function(data) {
      // Collect variant nodes (skip base nodes with empty variant)
      var variantNodes = data.nodes.filter(function(n) {
        return n.variant && Object.keys(n.variant).length > 0;
      });

      // Group by problem name
      var problems = {};
      variantNodes.forEach(function(n) {
        if (!problems[n.name]) {
          problems[n.name] = { category: n.category, doc_path: n.doc_path, children: [] };
        }
        problems[n.name].children.push(n);
      });

      // Build edges at variant level, detecting bidirectional pairs
      var edgeMap = {};
      data.edges.forEach(function(e) {
        var srcId = variantId(e.source.name, e.source.variant);
        var dstId = variantId(e.target.name, e.target.variant);
        var fwd = srcId + '->' + dstId;
        var rev = dstId + '->' + srcId;
        if (edgeMap[rev]) { edgeMap[rev].bidirectional = true; }
        else if (!edgeMap[fwd]) {
          edgeMap[fwd] = { source: srcId, target: dstId, bidirectional: false, overhead: e.overhead || [], doc_path: e.doc_path || '' };
        }
      });

      // Precompute per-problem base/non-base split
      var problemNames = Object.keys(problems);
      var problemInfo = {};
      problemNames.forEach(function(name) {
        var info = problems[name];
        var baseChild = null, nonBase = [];
        info.children.forEach(function(child) {
          if (isBaseVariant(child.variant)) baseChild = child;
          else nonBase.push(child);
        });
        problemInfo[name] = { baseChild: baseChild, nonBase: nonBase };
      });

      // ── Step 1: Layout one node per problem using cose ──
      var tempElements = [];
      problemNames.forEach(function(name) {
        var info = problems[name];
        tempElements.push({
          data: { id: name, label: name, category: info.category }
        });
      });
      var tempEdgeSet = {};
      data.edges.forEach(function(e) {
        var key = e.source.name + '->' + e.target.name;
        var rev = e.target.name + '->' + e.source.name;
        if (!tempEdgeSet[key] && !tempEdgeSet[rev]) {
          tempEdgeSet[key] = true;
          tempElements.push({ data: { id: 'te_' + key, source: e.source.name, target: e.target.name } });
        }
      });

      var tempCy = cytoscape({
        container: document.getElementById('cy'),
        elements: tempElements,
        style: [
          { selector: 'node', style: {
            'label': 'data(label)', 'text-valign': 'center', 'text-halign': 'center',
            'font-size': '11px', 'font-family': 'monospace',
            'width': function(ele) { return Math.max(ele.data('label').length * 7 + 14, 60); },
            'height': 28, 'shape': 'round-rectangle'
          }},
          { selector: 'edge', style: { 'width': 1, 'line-color': '#ddd', 'target-arrow-shape': 'none' } }
        ],
        layout: {
          name: 'cose', animate: false,
          nodeRepulsion: function() { return 16000; },
          idealEdgeLength: function() { return 200; },
          gravity: 0.15, numIter: 1000, padding: 40
        }
      });

      var positions = {};
      tempCy.nodes().forEach(function(n) {
        positions[n.id()] = { x: n.position('x'), y: n.position('y') };
      });
      tempCy.destroy();

      // ── Step 2: Place flat variant nodes near parent positions ──
      var elements = [];
      var variantOffsetY = 30;

      problemNames.forEach(function(name) {
        var info = problems[name];
        var pi = problemInfo[name];
        var pos = positions[name];

        if (pi.baseChild) {
          // Base variant at parent position, labeled with problem name
          var baseId = variantId(name, pi.baseChild.variant);
          elements.push({
            data: { id: baseId, label: name, category: info.category, doc_path: info.doc_path },
            position: { x: pos.x, y: pos.y }
          });
          // Non-base variants placed below
          pi.nonBase.forEach(function(child, i) {
            var vid = variantId(name, child.variant);
            var vl = variantLabel(child.variant);
            elements.push({
              data: { id: vid, label: name + ' (' + vl + ')', category: child.category, doc_path: child.doc_path },
              position: { x: pos.x, y: pos.y + (i + 1) * variantOffsetY }
            });
          });
        } else if (pi.nonBase.length === 1) {
          // Single non-base variant — place at parent position with just problem name
          var child = pi.nonBase[0];
          var vid = variantId(name, child.variant);
          elements.push({
            data: { id: vid, label: name, category: child.category, doc_path: child.doc_path },
            position: { x: pos.x, y: pos.y }
          });
        } else {
          // Multiple non-base variants, no base — first at parent, rest below
          pi.nonBase.forEach(function(child, i) {
            var vid = variantId(name, child.variant);
            var vl = variantLabel(child.variant);
            elements.push({
              data: { id: vid, label: name + ' (' + vl + ')', category: child.category, doc_path: child.doc_path },
              position: { x: pos.x, y: pos.y + i * variantOffsetY }
            });
          });
        }
      });

      // ── Step 3: Connect edges ──
      Object.keys(edgeMap).forEach(function(k) {
        var e = edgeMap[k];
        elements.push({
          data: {
            id: k, source: e.source, target: e.target,
            bidirectional: e.bidirectional, overhead: e.overhead, doc_path: e.doc_path
          }
        });
      });

      var cy = cytoscape({
        container: document.getElementById('cy'),
        elements: elements,
        style: [
          { selector: 'node', style: {
            'label': 'data(label)', 'text-valign': 'center', 'text-halign': 'center',
            'font-size': '10px', 'font-family': 'monospace',
            'width': function(ele) { return Math.max(ele.data('label').length * 6.5 + 10, 50); },
            'height': 24, 'shape': 'round-rectangle',
            'background-color': function(ele) { return categoryColors[ele.data('category')] || '#f0f0f0'; },
            'border-width': 1,
            'border-color': function(ele) { return categoryBorders[ele.data('category')] || '#999'; },
            'text-wrap': 'none', 'cursor': 'pointer'
          }},
          { selector: 'edge', style: {
            'width': 1.5, 'line-color': '#999', 'target-arrow-color': '#999', 'target-arrow-shape': 'triangle',
            'source-arrow-color': '#999',
            'source-arrow-shape': function(ele) { return ele.data('bidirectional') ? 'triangle' : 'none'; },
            'curve-style': 'bezier', 'arrow-scale': 0.7, 'cursor': 'pointer'
          }},
          { selector: '.highlighted', style: {
            'background-color': '#ff6b6b', 'border-color': '#cc0000', 'border-width': 2, 'z-index': 10
          }},
          { selector: 'edge.highlighted', style: {
            'line-color': '#ff4444', 'target-arrow-color': '#ff4444', 'source-arrow-color': '#ff4444', 'width': 3, 'z-index': 10
          }},
          { selector: '.selected-node', style: {
            'border-color': '#0066cc', 'border-width': 2, 'background-color': '#cce0ff'
          }}
        ],
        layout: { name: 'preset' },
        userZoomingEnabled: true, userPanningEnabled: true, boxSelectionEnabled: false
      });

      // Tooltip for nodes
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
        var html = '<strong>' + evt.target.source().data('label') + arrow + evt.target.target().data('label') + '</strong>';
        if (d.overhead && d.overhead.length > 0) {
          html += '<br>' + d.overhead.map(function(o) { return '<code>' + o.field + '</code> = <code>' + o.formula + '</code>'; }).join('<br>');
        }
        html += '<br><em>Click to highlight, double-click for source code</em>';
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

      // Double-click node → rustdoc API page
      cy.on('dbltap', 'node', function(evt) {
        var d = evt.target.data();
        if (d.doc_path) {
          window.location.href = 'api/problemreductions/' + d.doc_path;
        }
      });
      // Double-click edge → GitHub source code
      cy.on('dbltap', 'edge', function(evt) {
        var d = evt.target.data();
        if (d.doc_path) {
          var module = d.doc_path.replace('/index.html', '');
          window.open('https://github.com/CodingThrust/problem-reductions/blob/main/src/' + module + '.rs', '_blank');
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
        var text = edge.source().data('label') + arrow + edge.target().data('label');
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

## Our vision

In the past, computational complexity theorists focus mostly on the theory part, left the tedious implementation to developers. This `algorithm -> paper` mode causes many repeated efforts in algorithm implementation. Due to the lack of infrastructures, many fundamental issues can not be answered clearly, such as
- What is the fastest algorithm for solving a problem?
- Given an efficient solver, what are the potential high impact problems it can be used for?

Imagine, if every problem in this world is connected as a directed reduction graph, such that given any pairs of problems, we can easily find the most efficient reduction path between them. We can create a high speed rail-way between any two problems. We will no longer have repeated efforts in solving problems that are essentially the "same". However, developing this software infrastructure is not easy.

What if AI take over the implementation part, and complete the `algoirthm -> paper -> software` pipeline? Theorists can still focus on the theory part, and AI will do the heavy lifting. How is it possible? Can we trust AI's implementation? Our answer is, yes. Not only due to the fast evolving large language models brings new power, but also due to the fact that almost all reductions can be verified by running round-trip tests (`source -> target -> solution to target -> solution to source` must equal to `source -> solution to source`). We can easily verify the correctness of the reduction by checking round-trip reduction examples.

Our vision is to automate this test-driven development pipeline, and enable general public to contribute. Our software will be open sourced, forever, for at any physical location in the universe to every human being and AI agent.

## License

MIT License
