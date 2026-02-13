# Problem Reductions

**problem-reductions** is a rust library that provides implementations of various computational hard problems and reduction rules between them. It is designed for algorithm research, education, and industry applications.

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

  // Default values per variant key — omitted in concise labels
  var variantDefaults = { graph: 'SimpleGraph', weight: 'Unweighted' };

  function variantLabel(variant) {
    var keys = Object.keys(variant);
    var parts = [];
    keys.forEach(function(k) {
      var v = variant[k];
      if (variantDefaults[k] && v === variantDefaults[k]) return; // skip defaults
      parts.push(k === 'graph' || k === 'weight' ? v : k + '=' + v);
    });
    return parts.length > 0 ? parts.join(', ') : 'base';
  }

  function fullVariantLabel(variant) {
    var keys = Object.keys(variant);
    if (keys.length === 0) return 'no parameters';
    var parts = [];
    keys.forEach(function(k) {
      parts.push(k === 'graph' || k === 'weight' ? variant[k] : k + '=' + variant[k]);
    });
    return parts.join(', ');
  }

  function isBaseVariant(variant) {
    var keys = Object.keys(variant);
    return keys.every(function(k) {
      return variantDefaults[k] && variant[k] === variantDefaults[k];
    });
  }

  fetch('reductions/reduction_graph.json')
    .then(function(r) { if (!r.ok) throw new Error('HTTP ' + r.status); return r.json(); })
    .then(function(data) {
      // Group all nodes by problem name
      var problems = {};
      data.nodes.forEach(function(n) {
        if (!problems[n.name]) {
          problems[n.name] = { category: n.category, doc_path: n.doc_path, children: [] };
        }
        // Only track nodes with non-empty variants as children;
        // empty-variant nodes are base placeholders
        if (n.variant && Object.keys(n.variant).length > 0) {
          problems[n.name].children.push(n);
        }
      });

      // Build edges at variant level, detecting bidirectional pairs
      var edgeMap = {};
      data.edges.forEach(function(e) {
        var src = data.nodes[e.source];
        var dst = data.nodes[e.target];
        var srcId = variantId(src.name, src.variant);
        var dstId = variantId(dst.name, dst.variant);
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
        var srcName = data.nodes[e.source].name;
        var dstName = data.nodes[e.target].name;
        var key = srcName + '->' + dstName;
        var rev = dstName + '->' + srcName;
        if (!tempEdgeSet[key] && !tempEdgeSet[rev]) {
          tempEdgeSet[key] = true;
          tempElements.push({ data: { id: 'te_' + key, source: srcName, target: dstName } });
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

        if (info.children.length === 0) {
          // No parameterized variants — single node with empty variant
          var vid = variantId(name, {});
          elements.push({
            data: { id: vid, label: name, fullLabel: name + ' (no parameters)', category: info.category, doc_path: info.doc_path },
            position: { x: pos.x, y: pos.y }
          });
        } else if (pi.baseChild) {
          // Base variant at parent position, labeled with problem name
          var baseId = variantId(name, pi.baseChild.variant);
          elements.push({
            data: { id: baseId, label: name, fullLabel: name + ' (' + fullVariantLabel(pi.baseChild.variant) + ')', category: info.category, doc_path: info.doc_path },
            position: { x: pos.x, y: pos.y }
          });
          // Non-base variants placed below
          pi.nonBase.forEach(function(child, i) {
            var vid = variantId(name, child.variant);
            var vl = variantLabel(child.variant);
            elements.push({
              data: { id: vid, label: name + ' (' + vl + ')', fullLabel: name + ' (' + fullVariantLabel(child.variant) + ')', category: child.category, doc_path: child.doc_path },
              position: { x: pos.x, y: pos.y + (i + 1) * variantOffsetY }
            });
          });
        } else if (pi.nonBase.length === 1) {
          // Single non-base variant — place at parent position with just problem name
          var child = pi.nonBase[0];
          var vid = variantId(name, child.variant);
          elements.push({
            data: { id: vid, label: name, fullLabel: name + ' (' + fullVariantLabel(child.variant) + ')', category: child.category, doc_path: child.doc_path },
            position: { x: pos.x, y: pos.y }
          });
        } else {
          // Multiple non-base variants, no base — first at parent, rest below
          pi.nonBase.forEach(function(child, i) {
            var vid = variantId(name, child.variant);
            var vl = variantLabel(child.variant);
            elements.push({
              data: { id: vid, label: name + ' (' + vl + ')', fullLabel: name + ' (' + fullVariantLabel(child.variant) + ')', category: child.category, doc_path: child.doc_path },
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
        tooltip.innerHTML = '<strong>' + d.fullLabel + '</strong><br><em>Double-click to view API docs</em>';
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

For theoretical background and correctness proofs, see the [PDF manual](https://codingthrust.github.io/problem-reductions/reductions.pdf).

## Our Vision

Computational complexity theory has produced a rich body of polynomial-time reductions between NP-hard problems, yet these results largely remain confined to papers. The gap between theoretical algorithms and working software leads to two persistent inefficiencies:

- **Solver underutilization.** State-of-the-art solvers (SAT solvers, ILP solvers, QUBO annealers) each target a single problem formulation. In principle, any problem reducible to that formulation can leverage the same solver — but without a systematic reduction library, practitioners must re-derive and re-implement each transformation.
- **Redundant effort.** Problems that are polynomial-time equivalent are, from a computational standpoint, interchangeable. Without infrastructure connecting them, the same algorithmic insights are independently reimplemented across domains.

Our goal is to build a comprehensive, machine-readable reduction graph: a directed graph in which every node is a computational problem and every edge is a verified polynomial-time reduction. Given such a graph, one can automatically compose reduction paths to route any source problem to any reachable target solver.

A key enabler is AI-assisted implementation. We propose a pipeline of `algorithm → paper → software`, in which AI agents translate published reduction proofs into tested code. The critical question — can AI-generated reductions be trusted? — has a concrete answer: nearly all reductions admit **closed-loop verification**. A round-trip test reduces a source instance to a target, solves the target, extracts the solution back, and checks it against a direct solve of the source. This property makes correctness mechanically verifiable, independent of how the code was produced.

<div class="theme-light-only">

![](static/workflow-loop.svg)

</div>
<div class="theme-dark-only">

![](static/workflow-loop-dark.svg)

</div>

This library is the foundation of that effort: an open-source, extensible reduction graph with verified implementations, designed for contributions from both human researchers and AI agents.

## Call for Contributions

> **Everyone can contribute — no programming experience required.** If you know a computational problem or a reduction rule, just describe it in a GitHub issue. AI will generate a tested pull request for you to review.
>
> **Contribute 10 non-trivial reduction rules and you will be automatically added to the author list of the [paper](https://codingthrust.github.io/problem-reductions/reductions.pdf).**

1. **Open an issue** using the [Problem](https://github.com/CodingThrust/problem-reductions/issues/new?template=problem.md) or [Rule](https://github.com/CodingThrust/problem-reductions/issues/new?template=rule.md) template
2. **Fill in all sections** — definition, algorithm, size overhead, example instance
3. **Review AI-generated code** — AI generates code and you can comment on the pull request
4. **Merge** — ask maintainers' assistance to merge once you are satisfied

For manual implementation, see the [Architecture](./arch.md#contributing) guide.

## License

MIT License
